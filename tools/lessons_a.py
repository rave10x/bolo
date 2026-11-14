STAGE("s0", "Stage 1 · Start here", "Shuruaat", "Letters, sounds aur padhne ka tareeka")
STAGE("s1", "Stage 2 · Building blocks", "Sentence ke buniyadi hisse", "Pronouns, am/is/are, a/an/the, questions")
STAGE("s2", "Stage 3 · Tenses", "Kaal (time) ke sentences", "Present, past aur future")
STAGE("s3", "Stage 4 · Real conversation", "Asli baat-cheet", "Introduce, request, opinion, connectors")
STAGE("s4", "Stage 5 · Work & tech English", "Job aur coding wali English", "Interview, stand-up, email, bug explain")
STAGE("s5", "Stage 6 · Smart English", "Sahi aur natural English", "Common galtiyan aur fluency")

LESSON("l01", "s0", "Letters and their names", "26 letters ka naam kaise bolte hain", [
    P("English mein **26 letters** hote hain. Inme **5 vowels** (a, e, i, o, u) aur **21 consonants** hain. Hindi mein jaise 'क ख ग' hota hai, waise hi English mein A B C hai — bas inka **naam** aur **awaaz** alag hoti hai."),
    P("Sabse pehle letters ke **naam** pakke karo. Kisi ko apna naam spell karke batana ho, phone par email bolna ho, ya form bharna ho — har jagah ye kaam aata hai."),
    TBL(["Letter", "Naam (Hindi mein)", "Example word"],
        ["A a", "ए", "apple"], ["B b", "बी", "book"], ["C c", "सी", "cat"], ["D d", "डी", "dog"], ["E e", "ई", "egg"],
        ["F f", "एफ", "fish"], ["G g", "जी", "girl"], ["H h", "एच", "house"], ["I i", "आई", "ice"], ["J j", "जे", "juice"],
        ["K k", "के", "key"], ["L l", "एल", "lamp"], ["M m", "एम", "man"], ["N n", "एन", "nose"], ["O o", "ओ", "orange"],
        ["P p", "पी", "pen"], ["Q q", "क्यू", "queen"], ["R r", "आर", "rice"], ["S s", "एस", "sun"], ["T t", "टी", "tea"],
        ["U u", "यू", "umbrella"], ["V v", "वी", "van"], ["W w", "डबल-यू", "water"], ["X x", "एक्स", "x-ray"], ["Y y", "वाई", "yes"], ["Z z", "ज़ेड", "zoo"]),
    TIP("Har word ke saath speaker button dabao aur dhyan se suno. Phir khud bolo. Sirf padhna kaafi nahi — **suno aur bolo**."),
    H("Apna naam spell karna"),
    P("Koi poochhe 'How do you spell your name?' to har letter ka naam bolo."),
    EX("How do you spell your name? | आप अपना नाम कैसे spell करते हैं?",
       "R-A-H-U-L. | आर-ए-एच-यू-एल।",
       "My email is rahul at gmail dot com. | मेरा ईमेल rahul at gmail dot com है।",
       "Can you spell that, please? | क्या आप ये spell कर सकते हैं?"),
    H("Capital aur small letters"),
    P("**Capital letter** (A, B, C) in jagah use hote hain: sentence ke shuru mein, naam ke shuru mein (Rahul, India, Delhi), aur **I** (main) hamesha capital. Baaki jagah small letters (a, b, c)."),
    EX("My name is Rahul. | मेरा नाम राहुल है।", "I live in Delhi. | मैं दिल्ली में रहता हूँ।"),
], [
    Q("'B' ka naam kaise bola jata hai?", ["बी (bee)", "भी", "बे", "डी"], 0, "B ko 'बी' bolte hain."),
    Q("English mein kul kitne letters hote hain?", ["24", "26", "28", "52"], 1, "A se Z tak 26 letters."),
    Q("Kaun sa letter hamesha capital likha jata hai jab uska matlab 'main' ho?", ["i", "I", "a", "m"], 1, "'I' (main) hamesha capital hota hai."),
    Q("Naam ka pehla letter kaisa likhte hain?", ["Small", "Capital", "Kuch bhi", "Number"], 1, "Rahul, Delhi, India — naam ka pehla letter capital."),
    Q("Vowels kaun se hain?", ["a e i o u", "b c d f g", "x y z", "s t r"], 0, "a, e, i, o, u — baaki sab consonants."),
])

LESSON("l02", "s0", "Sounds that are tricky for Hindi speakers", "V/W, th, z, aur school wali galti", [
    P("Hindi bolne walon ko kuch awaazein English mein mushkil lagti hain. Ye lesson unhi ke baare mein hai. Dheere dheere practice karo — slow button se dheere suno."),
    H("1. V aur W alag hain"),
    P("Hindi mein 'व' ek hi awaaz hai, par English mein do hain. **V**: upar ke daant neeche ke hont par rakho (vet, van). **W**: hont gol karo, daant mat lagao (wet, water)."),
    EX("very | बहुत (V — daant hont par)", "wet | गीला (W — hont gol)", "vest and west | बनियान और पश्चिम", "I want water. | मुझे पानी चाहिए।"),
    H("2. 'th' ki do awaazein"),
    P("'th' mein **jeebh (tongue) daanton ke beech** thodi nikalo. Ye न 'त' hai न 'द'. Do tarah ki awaaz: halki (**think, three, thank**) aur bhaari (**this, that, mother**)."),
    EX("think | सोचना", "thank you | धन्यवाद", "this and that | यह और वह", "My mother and father | मेरी माँ और पिता"),
    H("3. S + consonant: shuru mein 'i' mat lagao"),
    P("Hindi mein 'school' ko aksar 'इस्कूल' bol dete hain. English mein shuru mein sirf **s** hai: **sk**ool, **st**op, **sp**eak. Pehle 's' ko lamba kheencho, phir aage badho."),
    EX("school | स्कूल (iskool nahi)", "station | स्टेशन (istation nahi)", "speak | बोलना (ispeak nahi)", "student | विद्यार्थी"),
    H("4. Z aur J alag hain"),
    P("**Z** mein dant ke paas 'ज़' ki bhinbhinahat. **J** mein 'ज' jaisi awaaz. zoo (ज़ू) aur Jew nahi — zoo aur **juice** alag hain."),
    EX("zoo | चिड़ियाघर", "juice | जूस", "zero | शून्य", "just | बस / सिर्फ़"),
    TIP("Roz 5 minute ek awaaz par practice karo. Apni awaaz record karke suno (Bolo section mein) — sabse tez sudhaar yahi se hota hai."),
], [
    Q("'wet' mein kaun si awaaz hai?", ["V (daant hont par)", "W (hont gol)", "B", "P"], 1, "W mein hont gol hote hain, daant hont par nahi lagte."),
    Q("'school' ko sahi kaise bolte hain?", ["इस्कूल", "स्कूल", "सकूल", "सुकूल"], 1, "S+k sidha: स्कूल, shuru mein 'i' nahi."),
    Q("'think' mein 'th' kaise bolte hain?", ["त", "द", "jeebh daanton ke beech", "थ jaisa hont band karke"], 2, "Jeebh daanton ke beech halki nikalte hain."),
    Q("Kaun sa word 'V' se shuru hota hai?", ["water", "very", "wet", "wine"], 1, "'very' V se shuru hota hai, baaki W se."),
    Q("'station' ka sahi uchcharan?", ["istation", "station", "sattion", "estation"], 1, "S+t sidha — 'स्टेशन'."),
])

LESSON("l03", "s0", "Reading new words: vowel patterns and silent letters", "Spelling dekh kar awaaz kaise pata kare", [
    P("English mein spelling aur awaaz hamesha match nahi karte — par kuch **patterns** hain jo 80% words handle kar dete hain. Ye lesson wahi patterns sikhata hai."),
    H("Pattern 1: Magic E (lamba vowel)"),
    P("Agar word ke end mein **e** ho (jo bola nahi jata), to pehla vowel apna **naam** bolta hai (lamba). Bina e ke vowel chhota hota hai."),
    TBL(["Chhota", "Lamba (magic e)"], ["cap (कैप)", "cape (केप)"], ["bit (बिट)", "bite (बाइट)"], ["hop (हॉप)", "hope (होप)"], ["cut (कट)", "cute (क्यूट)"], ["mad (मैड)", "made (मेड)"]),
    EX("The cap is on the table. | कैप मेज़ पर है।", "I hope you are fine. | मुझे उम्मीद है कि आप ठीक हैं।", "Please do not bite. | कृपया काटो मत।"),
    H("Pattern 2: Do vowels saath-saath"),
    P("**ee, ea** = 'ई' (meet, eat). **oo** = 'ऊ' (food) ya 'उ' (book). **ai, ay** = 'ए' (rain, day). **oa, ow** = 'ओ' (boat, show)."),
    EX("meet and eat | मिलना और खाना", "food and book | खाना और किताब", "rain today | आज बारिश", "boat show | नाव का शो"),
    H("Pattern 3: Silent letters (jo bole nahi jate)"),
    TBL(["Word", "Silent letter", "Sahi uchcharan"], ["knife", "k", "नाइफ"], ["know", "k", "नो"], ["write", "w", "राइट"], ["doubt", "b", "डाउट"], ["listen", "t", "लिसन"], ["half", "l", "हाफ"], ["hour", "h", "आवर"]),
    EX("I know the answer. | मुझे जवाब पता है।", "Please listen carefully. | कृपया ध्यान से सुनिए।", "It takes one hour. | इसमें एक घंटा लगता है।"),
    H("Stress: kaun si syllable zor se bolein"),
    P("Lambe words mein ek hissa zyada zor se bola jata hai. Jaise **BANana → ba-NA-na**, **teacher → TEA-cher**, **computer → com-PU-ter**. Naya word seekhte hi suno — zor kahan hai, notice karo."),
], [
    Q("'cape' mein 'a' kaisa bola jata hai?", ["chhota 'a' (cap)", "lamba 'ए' (केप)", "आ", "silent"], 1, "Magic e se pehla vowel apna naam bolta hai: केप."),
    Q("'knife' mein kaun sa letter silent hai?", ["k", "n", "i", "f"], 0, "K silent hai — 'नाइफ'."),
    Q("'hour' ka sahi uchcharan?", ["हौर (h bolkar)", "आवर", "होर", "हावर"], 1, "H silent hota hai: आवर."),
    Q("'ee' wale words kaise bolte hain?", ["ए", "ई", "ऊ", "ओ"], 1, "meet, see — 'ई'."),
    Q("'bite' aur 'bit' mein fark kya hai?", ["Koi nahi", "bite mein lamba 'आई' hai", "bit mein lamba hai", "bite silent hai"], 1, "Magic e ki wajah se bite = बाइट (lamba)."),
])
