LESSON("l04", "s1", "Pronouns: I, you, he, she, it, we, they", "Naam ki jagah chhote shabd", [
    P("**Pronoun** wo chhota shabd hai jo naam ki jagah aata hai. 'Rahul is a boy. **He** is tall.' — yahan Rahul ki jagah **He**. Ye 7 shabd English ki neev hain, inhe pakka kar lo."),
    TBL(["Pronoun", "Matlab", "Kab use karein"],
        ["I", "मैं", "Khud ke liye (hamesha capital)"], ["you", "तुम / आप", "Samne wale ke liye (ek ya kai)"], ["he", "वह (लड़का)", "Ek aadmi/ladka"],
        ["she", "वह (लड़की)", "Ek aurat/ladki"], ["it", "वह (चीज़/जानवर)", "Cheez, jaanwar, situation"], ["we", "हम", "Main + doosre log"], ["they", "वे", "Kai log ya cheezein"]),
    EX("I am a student. | मैं विद्यार्थी हूँ।", "You are my friend. | तुम मेरे दोस्त हो।", "He works in a bank. | वह बैंक में काम करता है।", "She is my sister. | वह मेरी बहन है।",
       "It is a big city. | यह एक बड़ा शहर है।", "We live in Delhi. | हम दिल्ली में रहते हैं।", "They are good players. | वे अच्छे खिलाड़ी हैं।"),
    TIP("Hindi mein 'tum' aur 'aap' do hain, par English mein dono ke liye sirf **you**. Respect 'please', 'sir', 'madam' se dikhate hain."),
    H("Object pronouns: me, him, her, us, them"),
    P("Jab pronoun kaam ka **receiver** ho (jis par kaam hota hai), to ye form lagta hai: I→**me**, he→**him**, she→**her**, we→**us**, they→**them**. 'You' aur 'it' badalte nahi."),
    EX("Please help me. | कृपया मेरी मदद कीजिए।", "I know him. | मैं उसे जानता हूँ।", "Call her now. | उसे अभी फ़ोन करो।", "He teaches us. | वह हमें पढ़ाता है।"),
    FIX("Me and him are friends.", "He and I are friends.", "Subject (kaam karne wala) ki jagah I/he aata hai, me/him nahi."),
], [
    Q("'Rahul is tall. ___ plays cricket.'", ["He", "She", "It", "They"], 0, "Rahul ladka hai to He."),
    Q("Apne liye hamesha kaun sa pronoun capital hota hai?", ["me", "I", "we", "my"], 1, "'I' hamesha capital."),
    Q("'Please help ___.' (mujhe)", ["I", "me", "my", "mine"], 1, "Receiver ke liye 'me'."),
    Q("'The dog is hungry. ___ wants food.'", ["He", "It", "They", "We"], 1, "Jaanwar/cheez ke liye aam taur par 'It'."),
    Q("Kai log ke liye?", ["he", "she", "they", "it"], 2, "Kai log = they."),
])

LESSON("l05", "s1", "am, is, are — the 'to be' verb", "'hai/hoon/ho' ko English mein kya kehte hain", [
    P("Hindi mein 'main student **hoon**', 'woh doctor **hai**' — ye 'hoon/hai/ho' English mein **am / is / are** banta hai. **Har English sentence mein verb zaroori hai**, isliye 'I student' galat hai — 'I **am** a student' sahi hai."),
    TBL(["Subject", "Verb", "Example"], ["I", "am", "I am happy."], ["he / she / it", "is", "She is a doctor."], ["you / we / they", "are", "They are friends."]),
    EX("I am a student. | मैं विद्यार्थी हूँ।", "He is my brother. | वह मेरा भाई है।", "It is a nice day. | आज अच्छा दिन है।", "We are ready. | हम तैयार हैं।", "You are right. | आप सही हैं।"),
    H("Short forms (bolchaal mein)"),
    P("Bolte waqt hum chhota roop use karte hain: **I'm, he's, she's, it's, we're, you're, they're**."),
    EX("I'm fine. | मैं ठीक हूँ।", "She's at home. | वह घर पर है।", "They're late. | वे लेट हैं।"),
    H("Negative aur Question"),
    P("Negative: verb ke baad **not** lagao. Question: verb ko **subject se pehle** laao."),
    TBL(["Positive", "Negative", "Question"], ["He is tired.", "He is not tired. (isn't)", "Is he tired?"], ["They are here.", "They are not here. (aren't)", "Are they here?"], ["I am late.", "I am not late.", "Am I late?"]),
    EX("Is she your sister? | क्या वह तुम्हारी बहन है?", "Are you ready? | क्या तुम तैयार हो?", "I am not hungry. | मुझे भूख नहीं है।", "They aren't at home. | वे घर पर नहीं हैं।"),
    FIX("I student.", "I am a student.", "English mein verb (am) ke bina sentence poora nahi hota."),
    FIX("He are my friend.", "He is my friend.", "He/She/It ke saath hamesha is."),
], [
    Q("She ___ a teacher.", ["am", "is", "are", "be"], 1, "She ke saath is."),
    Q("They ___ my friends.", ["is", "am", "are", "be"], 2, "They ke saath are."),
    Q("I ___ happy today.", ["is", "are", "am", "be"], 2, "I ke saath am."),
    Q("Sahi question chuno.", ["You are ready?", "Are you ready?", "Is you ready?", "Ready are you?"], 1, "Verb subject se pehle: Are you ready?"),
    Q("'He is not here.' ka chhota roop?", ["He isn't here.", "He amn't here.", "He aren't here.", "He no here."], 0, "is not = isn't."),
])

LESSON("l06", "s1", "a, an, the (articles)", "Kab 'a', kab 'an', kab 'the'", [
    P("Hindi mein articles nahi hote, isliye ye sabse zyada galti wali jagah hai. Teen chhote shabd, teen simple rule."),
    H("a aur an — 'koi ek' (pehli baar baat)"),
    P("Ek ginti wali cheez ke liye **a/an** lagta hai. **a** vowel *sound* se pehle nahi, consonant sound se pehle: a book, a pen. **an** vowel *sound* (a, e, i, o, u awaaz) se pehle: an apple, an egg, an hour (h silent hai, isliye an!)."),
    EX("I have a pen. | मेरे पास एक कलम है।", "She eats an apple. | वह एक सेब खाती है।", "He is an engineer. | वह एक इंजीनियर है।", "It takes an hour. | इसमें एक घंटा लगता है।", "This is a university. | यह एक विश्वविद्यालय है। (yoo-sound, isliye a)"),
    H("the — 'wo khaas/jaani hui cheez'"),
    P("Jab sunne wala jaanta ho ki kaun si cheez ki baat hai, tab **the**. Pehli baar: 'I saw **a** dog.' Phir: '**The** dog was big.'"),
    EX("Close the door. | दरवाज़ा बंद करो।", "The sun is hot. | सूरज गरम है।", "I saw a movie. The movie was good. | मैंने एक फ़िल्म देखी। वह फ़िल्म अच्छी थी।"),
    H("Kab koi article nahi?"),
    P("Aam baat, plural ya uncountable cheez ke saath: 'I like **music**.' 'Dogs are loyal.' 'I drink **water**.' Languages, meals, aur zyadatar naam bhi: 'I speak English.' 'We have lunch at one.'"),
    FIX("I am student.", "I am a student.", "Ginti wale noun (student) se pehle a/an lagta hai."),
    FIX("She is an teacher.", "She is a teacher.", "teacher consonant sound se shuru hota hai, to 'a'."),
], [
    Q("___ apple a day is good.", ["A", "An", "The", "No article"], 1, "Apple vowel sound se shuru: an."),
    Q("He is ___ engineer.", ["a", "an", "the", "no article"], 1, "engineer vowel sound se shuru: an."),
    Q("I saw a dog. ___ dog was big.", ["A", "An", "The", "No"], 2, "Pehle se mention hui cheez: The."),
    Q("I like ___.", ["the music", "a music", "music", "an music"], 2, "Aam pasand (uncountable): bina article."),
    Q("It takes ___ hour.", ["a", "an", "the", "no article"], 1, "'hour' mein h silent, awaaz vowel jaisi: an hour."),
])

LESSON("l07", "s1", "Plural nouns", "Ek se zyada cheezein", [
    P("Ek cheez = **singular**, ek se zyada = **plural**. Zyadatar words mein plural banane ke liye bas **-s** lagta hai."),
    TBL(["Rule", "Singular", "Plural"], ["Seedha -s", "book", "books"], ["s, x, ch, sh, o ke baad -es", "box / watch / bus", "boxes / watches / buses"], ["Consonant + y → -ies", "baby / city", "babies / cities"], ["f/fe → -ves", "knife / leaf", "knives / leaves"]),
    H("Kuch ulte-seedhe (irregular) plural"),
    TBL(["Singular", "Plural"], ["man", "men"], ["woman", "women"], ["child", "children"], ["foot", "feet"], ["tooth", "teeth"], ["person", "people"]),
    EX("Two books | दो किताबें", "Three boxes | तीन डिब्बे", "Five babies | पाँच बच्चे", "Many children | बहुत सारे बच्चे", "These people are kind. | ये लोग दयालु हैं।"),
    H("Jo count nahi hote (uncountable)"),
    P("**water, rice, milk, money, news, information, advice, furniture, work, hair** — inka plural (-s) nahi banta, aur 'a/an' bhi nahi lagta. 'Much' aur 'some' lagta hai."),
    EX("I need some advice. | मुझे कुछ सलाह चाहिए।", "The news is good. | ख़बर अच्छी है।", "We have much work. | हमारे पास बहुत काम है।"),
    FIX("I need some informations.", "I need some information.", "Information uncountable hai: informations nahi."),
], [
    Q("box ka plural?", ["boxs", "boxes", "boxies", "boxen"], 1, "x ke baad -es."),
    Q("child ka plural?", ["childs", "childes", "children", "childrens"], 2, "child → children (irregular)."),
    Q("city ka plural?", ["citys", "cities", "cityes", "city"], 1, "consonant + y → ies."),
    Q("Sahi sentence?", ["I need some advices.", "I need some advice.", "I need an advice.", "I need advices."], 1, "advice uncountable."),
    Q("man ka plural?", ["mans", "manes", "men", "mens"], 2, "man → men."),
])

LESSON("l08", "s1", "this, that, these, those", "Paas aur door ki cheezein", [
    P("Kisi cheez ki taraf ishara karne ke liye 4 shabd. Do cheezein dekho: **paas ya door** aur **ek ya kai**."),
    TBL(["", "Paas (here)", "Door (there)"], ["Ek (singular)", "this (yah)", "that (wo)"], ["Kai (plural)", "these (ye sab)", "those (wo sab)"]),
    EX("This is my book. | यह मेरी किताब है।", "That is your bag. | वह तुम्हारा बैग है।", "These are my friends. | ये मेरे दोस्त हैं।", "Those are old houses. | वे पुराने मकान हैं।"),
    H("Noun ke saath"),
    EX("This pen is blue. | यह कलम नीली है।", "Those shoes are new. | वे जूते नए हैं।", "Who is that man? | वह आदमी कौन है?", "I like these mangoes. | मुझे ये आम पसंद हैं।"),
    TIP("Spelling: **this** (दिस) mein chhota 'इ', **these** (दीज़) mein lamba 'ई'. Dono ko alag bolna practice karo — dheere (slow) suno."),
    FIX("These is my friend.", "This is my friend.", "Ek vyakti = this. Plural ke saath these + are."),
], [
    Q("___ is my pen. (paas, ek)", ["This", "That", "These", "Those"], 0, "Paas + ek = This."),
    Q("___ are my shoes. (paas, kai)", ["This", "That", "These", "Those"], 2, "Paas + kai = These."),
    Q("___ birds are beautiful. (door, kai)", ["This", "That", "These", "Those"], 3, "Door + kai = Those."),
    Q("'That house' mein 'that' ka matlab?", ["yah (paas)", "wo (door)", "ye sab", "wo sab"], 1, "That = wo (door, ek)."),
    Q("Sahi sentence?", ["These book is new.", "This books are new.", "These books are new.", "This book are new."], 2, "These + plural noun + are."),
])

LESSON("l09", "s1", "have, has, there is / there are", "'Mere paas hai' aur 'wahan hai'", [
    P("**Mere paas hai** = **I have**. Hindi ka 'ke paas' English mein **have/has** ban jata hai. He/She/It ke saath **has**, baaki sab ke saath **have**."),
    EX("I have a phone. | मेरे पास फ़ोन है।", "She has two sisters. | उसकी दो बहनें हैं।", "We have a big house. | हमारा एक बड़ा घर है।", "He has a car. | उसके पास कार है।"),
    TBL(["Subject", "Verb"], ["I / you / we / they", "have"], ["he / she / it", "has"]),
    H("Negative aur question (do/does ke saath)"),
    EX("I do not have time. | मेरे पास समय नहीं है।", "She does not have a bike. | उसके पास बाइक नहीं है।", "Do you have a pen? | क्या तुम्हारे पास कलम है?", "Does he have a car? | क्या उसके पास कार है?"),
    H("there is / there are — 'wahan hai'"),
    P("Kisi jagah par kuch **hone** ki baat: ek cheez = **there is**, kai cheezein = **there are**."),
    EX("There is a book on the table. | मेज़ पर एक किताब है।", "There are three chairs here. | यहाँ तीन कुर्सियाँ हैं।", "Is there a bank near here? | क्या यहाँ पास में बैंक है?", "There are no clouds today. | आज बादल नहीं हैं।"),
    FIX("She have a car.", "She has a car.", "She/He/It ke saath has."),
    FIX("There is many books.", "There are many books.", "Plural ke saath there are."),
], [
    Q("He ___ a bike.", ["have", "has", "having", "is have"], 1, "He ke saath has."),
    Q("I ___ two brothers.", ["has", "have", "am have", "having"], 1, "I ke saath have."),
    Q("There ___ five students in the class.", ["is", "are", "has", "have"], 1, "Plural: there are."),
    Q("___ you have a pen?", ["Does", "Do", "Are", "Is"], 1, "You ke saath Do."),
    Q("'Mere paas time nahi hai' =", ["I not have time.", "I do not have time.", "I am not have time.", "I no has time."], 1, "do not have."),
])

LESSON("l10", "s1", "can, cannot, must, should", "Kar sakna, karna chahiye, karna hi hai", [
    P("**can** = sakna. **must** = zaroor karna. **should** = karna chahiye. In teeno ke baad **verb ka seedha roop** aata hai (to, -s, -ing kuch nahi). Sab subject ke liye same: I can, he can, they can."),
    EX("I can swim. | मैं तैर सकता हूँ।", "She can speak English. | वह अंग्रेज़ी बोल सकती है।", "You must wear a helmet. | आपको हेलमेट पहनना ही होगा।", "You should drink more water. | तुम्हें ज़्यादा पानी पीना चाहिए।"),
    H("Negative aur question"),
    EX("I cannot (can't) come today. | मैं आज नहीं आ सकता।", "He should not shout. | उसे चिल्लाना नहीं चाहिए।", "Can you help me? | क्या आप मेरी मदद कर सकते हैं?", "Should I wait? | क्या मुझे इंतज़ार करना चाहिए?"),
    TIP("Question banane ke liye **can/should ko subject se pehle** rakho: 'Can you...?', 'Should we...?'. 'Do' ki zaroorat nahi."),
    FIX("He can swims.", "He can swim.", "Can ke baad verb ka seedha roop: swim."),
    FIX("She can to drive.", "She can drive.", "Can ke baad 'to' nahi aata."),
], [
    Q("She ___ speak English.", ["can", "cans", "can to", "is can"], 0, "can + seedha verb."),
    Q("You ___ drink more water. (salah)", ["should", "can", "am", "have"], 0, "Salah = should."),
    Q("Sahi sentence?", ["He can plays.", "He can play.", "He cans play.", "He can to play."], 1, "can + play."),
    Q("'Mujhe aana chahiye?' =", ["Should I come?", "I should come?", "Do I should come?", "Am I should come?"], 0, "Should I come?"),
    Q("'Main nahi kar sakta' =", ["I can not to do", "I cannot do", "I do not can", "I am cannot"], 1, "I cannot do."),
])

LESSON("l11", "s1", "Question words: what, where, when, why, who, how", "Sawaal kaise poochhein", [
    P("Sawaal poochne ke liye 6 mukhya shabd. Inke saath sentence ka rule: **Question word + helping verb + subject**."),
    TBL(["Word", "Matlab", "Example"],
        ["what", "क्या", "What is your name?"], ["where", "कहाँ", "Where do you live?"], ["when", "कब", "When does the class start?"],
        ["why", "क्यों", "Why are you late?"], ["who", "कौन", "Who is that man?"], ["how", "कैसे", "How do you go to school?"]),
    EX("What do you do? | आप क्या करते हैं?", "Where is the station? | स्टेशन कहाँ है?", "When do you wake up? | आप कब उठते हैं?", "Why are you sad? | तुम उदास क्यों हो?", "How are you? | आप कैसे हैं?"),
    H("Kuch kaam ke combos"),
    TBL(["Combo", "Matlab", "Example"], ["how much", "kitna (cheez/paisa)", "How much is this?"], ["how many", "kitne (ginti)", "How many books do you have?"], ["how old", "kitni umar", "How old are you?"], ["what time", "kitne baje", "What time is it?"], ["how long", "kitni der", "How long does it take?"]),
    EX("How much is this shirt? | यह कमीज़ कितने की है?", "How many brothers do you have? | आपके कितने भाई हैं?", "How old are you? | आपकी उम्र क्या है?", "What time is the meeting? | मीटिंग कितने बजे है?"),
    FIX("You are where from?", "Where are you from?", "Question word sentence ke shuru mein aata hai."),
    FIX("What you want?", "What do you want?", "Helping verb 'do' zaroori hai."),
], [
    Q("___ is your name? ", ["Where", "What", "When", "How"], 1, "Naam ke liye What."),
    Q("___ do you live? (kahan)", ["Why", "Who", "Where", "How"], 2, "Jagah = Where."),
    Q("___ many books do you have?", ["How", "What", "Who", "When"], 0, "Kitne = How many."),
    Q("Sahi sentence?", ["What you want?", "What do you want?", "What want you?", "You what want?"], 1, "What + do + you + verb."),
    Q("'Aap ki umar kya hai?'", ["How old are you?", "What old are you?", "How many are you?", "Which age are you?"], 0, "How old are you?"),
])

LESSON("l12", "s1", "in, on, at (place and time)", "Ye teen chhote shabd sabse zyada confuse karte hain", [
    P("Hindi mein 'mein/par/pe' sab ke liye ek-do shabd hain, English mein **in, on, at** alag-alag kaam karte hain. Ek trick: **in** = andar (bada area/ghira hua), **on** = upar (surface par), **at** = ek point par."),
    H("Jagah (place)"),
    TBL(["Shabd", "Idea", "Example"], ["in", "andar / ghira hua", "in the room, in Delhi, in India"], ["on", "surface par", "on the table, on the wall, on the bus"], ["at", "ek point / jagah par", "at the door, at home, at school"]),
    EX("The milk is in the fridge. | दूध फ्रिज में है।", "The book is on the table. | किताब मेज़ पर है।", "I am at home. | मैं घर पर हूँ।", "She lives in Mumbai. | वह मुंबई में रहती है।"),
    H("Samay (time)"),
    TBL(["Shabd", "Kab", "Example"], ["at", "ghadi ka samay", "at 5 o'clock, at night, at noon"], ["on", "din / tareekh", "on Monday, on 15 August"], ["in", "mahina, saal, mausam, din ka hissa", "in May, in 2025, in winter, in the morning"]),
    EX("I wake up at six. | मैं छह बजे उठता हूँ।", "We meet on Sunday. | हम रविवार को मिलते हैं।", "My birthday is in May. | मेरा जन्मदिन मई में है।", "I study in the evening. | मैं शाम को पढ़ाई करता हूँ।"),
    TIP("Yaad rakhne ki line: **'at night' lekin 'in the morning'** — ye ulte lagte hain par yehi sahi hai."),
    FIX("I will meet you in Monday.", "I will meet you on Monday.", "Din ke naam ke saath on."),
    FIX("The class starts in 9 o'clock.", "The class starts at 9 o'clock.", "Ghadi ke samay ke saath at."),
], [
    Q("I wake up ___ six.", ["in", "on", "at", "by"], 2, "Ghadi ka samay: at."),
    Q("We have a test ___ Friday.", ["in", "on", "at", "to"], 1, "Din: on."),
    Q("I was born ___ 2002.", ["on", "at", "in", "of"], 2, "Saal: in."),
    Q("The keys are ___ the table.", ["in", "on", "at", "to"], 1, "Surface par: on."),
    Q("She lives ___ Delhi.", ["on", "at", "in", "to"], 2, "Shahar: in."),
])

LESSON("l13", "s1", "my, your, his, her — whose is it?", "'Mera, tumhara, uska' English mein", [
    P("Hindi mein 'mera/meri/mere' alag hote hain. English mein ek hi shabd: **my**. Ye shabd noun se **pehle** aata hai."),
    TBL(["Pronoun", "Noun se pehle", "Akela (noun ke bina)"], ["I", "my", "mine"], ["you", "your", "yours"], ["he", "his", "his"], ["she", "her", "hers"], ["it", "its", "—"], ["we", "our", "ours"], ["they", "their", "theirs"]),
    EX("This is my bag. | यह मेरा बैग है।", "Your phone is ringing. | तुम्हारा फ़ोन बज रहा है।", "His sister is a nurse. | उसकी बहन नर्स है।", "Her name is Priya. | उसका नाम प्रिया है।", "Our house is small. | हमारा घर छोटा है।", "Their car is red. | उनकी कार लाल है।"),
    EX("This bag is mine. | यह बैग मेरा है।", "Is this pen yours? | क्या यह कलम तुम्हारी है?"),
    H("'s se maalkiyat"),
    P("Kisi ke naam ke baad **'s** lagao: **Rahul's book** (राहुल की किताब), **my mother's phone** (मेरी माँ का फ़ोन)."),
    EX("This is Rahul's bag. | यह राहुल का बैग है।", "My friend's house is near. | मेरे दोस्त का घर पास में है।"),
    FIX("This is mine book.", "This is my book.", "Noun se pehle 'my', 'mine' sirf akela aata hai."),
    FIX("Me and my brother are friends.", "My brother and I are friends.", "Subject mein 'I' aata hai, 'me' nahi; aur apne aap ko aakhir mein rakhna shishtachar hai."),
], [
    Q("This is ___ book. (meri)", ["me", "my", "mine", "I"], 1, "Noun se pehle my."),
    Q("The pen is ___. (meri hai)", ["my", "me", "mine", "I"], 2, "Akela: mine."),
    Q("___ sister is a doctor. (uski, ladki ki)", ["His", "Her", "Their", "Its"], 1, "Ladki: her."),
    Q("This is Rahul___ bag.", ["s", "'s", "s'", "is"], 1, "Rahul's bag."),
    Q("'Hamara ghar' =", ["Us house", "Ours house", "Our house", "We house"], 2, "Our house."),
])
