#!/usr/bin/env python3
"""Builds content/*.json from tools/*.txt and tools/lessons_*.py. Run: python3 -I tools/build.py"""
import json, sys, os, re
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "..", "content")

def blocks_file(name, parse):
    path = os.path.join(HERE, name)
    return [l.rstrip("\n") for l in open(path, encoding="utf-8") if l.strip() and not l.startswith("#!")] if os.path.exists(path) else []

def words():
    topics, cur = [], None
    for l in blocks_file("words.txt", None):
        if l.startswith("## "):
            tid, name, hi, icon = [x.strip() for x in l[3:].split("|")]
            cur = {"id": tid, "name": name, "hi": hi, "icon": icon, "words": []}
            topics.append(cur)
        else:
            en, hi, pr, ex, exhi = [x.strip() for x in l.split("|")]
            cur["words"].append({"en": en, "hi": hi, "pr": pr, "ex": ex, "exhi": exhi})
    return {"topics": topics}

def phrases():
    packs, cur = [], None
    for l in blocks_file("phrases.txt", None):
        if l.startswith("## "):
            pid, name, hi, icon = [x.strip() for x in l[3:].split("|")]
            cur = {"id": pid, "name": name, "hi": hi, "icon": icon, "items": []}
            packs.append(cur)
        else:
            en, hi = [x.strip() for x in l.split("|")]
            cur["items"].append({"en": en, "hi": hi})
    return {"packs": packs}

def dialogues():
    items, cur = [], None
    for l in blocks_file("dialogues.txt", None):
        if l.startswith("## "):
            did, title, hi, icon, cat, role = [x.strip() for x in l[3:].split("|")]
            cur = {"id": did, "title": title, "hi": hi, "icon": icon, "cat": cat, "role": role, "lines": [], "keys": []}
            items.append(cur)
        elif l.startswith("Q: "):
            cur["lines"].append({"who": "q", "text": l[3:].strip()})
        elif l.startswith("Y: "):
            cur["lines"].append({"who": "y", "text": l[3:].strip()})
        elif l.startswith("K: "):
            en, hi = [x.strip() for x in l[3:].split("|")]
            cur["keys"].append({"en": en, "hi": hi})
        else:
            raise SystemExit("bad dialogue line: " + l)
    # longer conversations: extra exchanges from dialogues_more.txt are spliced in after line N
    byid = {x["id"]: x for x in items}
    cur, n, extra = None, 0, []
    def flush():
        if cur is not None:
            if n % 2 or n > len(cur["lines"]): raise SystemExit("bad insert point for " + cur["id"])
            cur["lines"][n:n] = extra
    for l in blocks_file("dialogues_more.txt", None):
        if l.startswith("## "):
            flush()
            did, n = l[3:].split("|"); n = int(n)
            if did not in byid: raise SystemExit("dialogues_more: unknown id " + did)
            cur, extra = byid[did], []
        elif l[:3] in ("Q: ", "Y: "):
            extra.append({"who": l[0].lower(), "text": l[3:].strip()})
        elif not l.startswith("# "):
            raise SystemExit("bad dialogues_more line: " + l)
    flush()
    return {"items": items}

def mistakes():
    items = []
    for l in blocks_file("mistakes.txt", None):
        w, r, why = [x.strip() for x in l.split("|")]
        items.append({"wrong": w, "right": r, "why": why})
    return {"items": items}

def _helpers():
    def P(x): return {"t": "p", "x": x}
    def H(x): return {"t": "h", "x": x}
    def TIP(x): return {"t": "tip", "x": x}
    def EX(*pairs):
        items = []
        for p in pairs:
            en, hi = [x.strip() for x in p.split("|")]
            items.append({"en": en, "hi": hi})
        return {"t": "ex", "items": items}
    def TBL(head, *rows): return {"t": "tbl", "head": list(head), "rows": [list(r) for r in rows]}
    def FIX(wrong, right, why): return {"t": "fix", "wrong": wrong, "right": right, "why": why}
    def Q(q, opts, ans, why): return {"q": q, "opts": list(opts), "ans": ans, "why": why}
    TR = {}
    def T(hi, en):
        TR[hi] = en
        return hi
    ns = {"TR": TR, "T": T, "P": P, "H": H, "TIP": TIP, "EX": EX, "TBL": TBL, "FIX": FIX, "Q": Q, "STAGES": [], "LESSONS": []}
    def STAGE(id, name, hi, desc): ns["STAGES"].append({"id": id, "name": name, "hi": hi, "desc": desc})
    def LESSON(id, stage, title, hi, blocks, quiz): ns["LESSONS"].append({"id": id, "stage": stage, "title": title, "hi": hi, "blocks": blocks, "quiz": quiz})
    ns["STAGE"] = STAGE; ns["LESSON"] = LESSON
    return ns

def lessons():
    ns = _helpers()
    for fn in sorted(os.listdir(HERE)):
        if fn.startswith("lessons_") and fn.endswith(".py"):
            exec(compile(open(os.path.join(HERE, fn), encoding="utf-8").read(), fn, "exec"), ns)
    json.dump(ns["TR"], open(os.path.join(HERE, "en_auto.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=0)
    return {"stages": ns["STAGES"], "lessons": ns["LESSONS"]}

def calls():
    """Call-interview response drill: tools/calls_drill.py defines ITEMS via Q(...)."""
    ns = _helpers()
    ns["ITEMS"] = []
    exec(compile(open(os.path.join(HERE, "calls_drill.py"), encoding="utf-8").read(), "calls_drill.py", "exec"), ns)
    tr = json.load(open(os.path.join(HERE, "en_auto.json"), encoding="utf-8"))
    tr.update(ns["TR"])
    json.dump(tr, open(os.path.join(HERE, "en_auto.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=0)
    return {"items": ns["ITEMS"]}

def check(w, p, m, l, d):
    errs = []
    seen = set()
    for t in w["topics"]:
        for x in t["words"]:
            k = (t["id"], x["en"].lower())
            if k in seen: errs.append(f"duplicate word {k}")
            seen.add(k)
            if not re.search(r"(?<![A-Za-z])" + re.escape(x["en"].lower()) + r"(?![A-Za-z])", x["ex"].lower()):
                errs.append(f"word not in example as whole word: {x['en']} / {x['ex']}")
    sids = {s["id"] for s in l["stages"]}
    lids = set()
    for x in l["lessons"]:
        if x["id"] in lids: errs.append("dup lesson " + x["id"])
        lids.add(x["id"])
        if x["stage"] not in sids: errs.append("bad stage " + x["stage"])
        for q in x["quiz"]:
            if not (0 <= q["ans"] < len(q["opts"])): errs.append("bad ans in " + x["id"])
            if len(set(q["opts"])) != len(q["opts"]): errs.append("dup opts in " + x["id"] + ": " + q["q"])
        for b in x["blocks"]:
            if b["t"] == "ex" and not b["items"]: errs.append("empty ex " + x["id"])
    for it in m["items"]:
        if it["wrong"] == it["right"]: errs.append("same mistake " + it["wrong"])
    ids = set()
    for x in d["items"]:
        if x["id"] in ids: errs.append("dup dialogue " + x["id"])
        ids.add(x["id"])
        whos = [t["who"] for t in x["lines"]]
        if "q" not in whos or "y" not in whos: errs.append("dialogue needs both roles: " + x["id"])
        if whos[0] != "q": errs.append("dialogue should start with partner: " + x["id"])
        if not x["keys"]: errs.append("no key phrases: " + x["id"])
    return errs

if __name__ == "__main__":
    w, p, m, l, d = words(), phrases(), mistakes(), lessons(), dialogues()
    c = calls()
    errs = check(w, p, m, l, d)
    if errs:
        print("\n".join(errs)); sys.exit(1)
    os.makedirs(OUT, exist_ok=True)
    for name, data in [("words", w), ("phrases", p), ("mistakes", m), ("lessons", l), ("dialogues", d), ("calls", c)]:
        json.dump(data, open(os.path.join(OUT, name + ".json"), "w", encoding="utf-8"), ensure_ascii=False, separators=(",", ":"))
    nw = sum(len(t["words"]) for t in w["topics"]); np_ = sum(len(x["items"]) for x in p["packs"])
    print(f"words {nw} in {len(w['topics'])} topics | phrases {np_} in {len(p['packs'])} packs | mistakes {len(m['items'])} | lessons {len(l['lessons'])} in {len(l['stages'])} stages | dialogues {len(d['items'])} ({sum(len(x['lines']) for x in d['items'])} lines)")
