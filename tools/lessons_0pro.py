STAGE("s6", "Professional speaking", "Office aur interview ki English", "Intermediate: natural, confident workplace English (basics neeche Stage 1-6 mein hain)")

LESSON("p01", "s6", "Sounding professional: soften and hedge", "Seedhi baat ko tameez se kehna", [
    P("Office English mein seedha 'No' ya 'You are wrong' rude lagta hai. Native professionals baat ko **soften** karte hain — ye ek hi skill aapki English ko turant professional bana deti hai."),
    TBL(["Seedha (rough)", "Professional (soft)"],
        ["Give me the report.", "Could you please share the report when you get a chance?"],
        ["You are wrong.", "I see it a little differently."],
        ["I cannot do this.", "I am afraid I will not be able to finish this by Friday."],
        ["Why are you late?", "Is everything okay? We were expecting you at ten."],
        ["I don't understand.", "Sorry, could you explain that once more?"],
        ["This is bad.", "I think there is room for improvement here."]),
    EX("I was wondering if you could review my code today. | मैं सोच रहा था कि क्या आप आज मेरा code देख सकते हैं।", "It might be better to split this into two tasks. | इसे दो कामों में बाँटना बेहतर हो सकता है।", "I would suggest we test it first. | मैं सुझाव दूँगा कि पहले test करें।", "Would it be possible to extend the deadline by a day? | क्या deadline एक दिन बढ़ना संभव होगा?", "To be honest, I am not completely sure about that. | सच कहूँ तो मुझे इसका पूरा यक़ीन नहीं।"),
    H("Hedging words"),
    P("**might, could, would, perhaps, I think, it seems, a little, slightly, probably** — inse baat narm ho jaati hai aur galti hone par bhi aap safe rehte ho."),
    TIP("**'I think' ke saath 'but' ka use**: 'I think this is a good idea, but we may need more time.' Pehle agree, phir concern — ye formula har meeting mein kaam karta hai."),
    H("Thanks aur apology bhi professional"),
    EX("Thank you for bringing this up. | यह उठाने के लिए धन्यवाद।", "I apologise for the delay, and thank you for your patience. | देरी के लिए माफ़ी, और धैर्य के लिए धन्यवाद।", "I appreciate your quick response. | आपके जल्दी जवाब की मैं सराहना करता हूँ।"),
], [
    Q("Sabse professional request?", ["Send me the file now.", "Could you please send me the file when you get a chance?", "I want the file.", "File send."], 1, "Could you please ... when you get a chance = soft aur polite."),
    Q("'You are wrong' ki jagah?", ["You are not right at all.", "I see it a little differently.", "No!", "Wrong."], 1, "Disagree karte waqt 'I see it a little differently' soft hai."),
    Q("Kaun sa hedging word hai?", ["definitely", "perhaps", "never", "must"], 1, "perhaps = shayad, narm karta hai."),
    Q("Deadline badhwane ka polite tareeka?", ["Give me more time.", "Would it be possible to extend the deadline by a day?", "I need time.", "Deadline is bad."], 1, "Would it be possible ...?"),
    Q("Agree + concern formula?", ["I think this is good, but we may need more time.", "No, this is wrong.", "Good, bye.", "I do not care."], 0, "Pehle agree, phir concern."),
])

LESSON("p02", "s6", "STAR: answering behavioural questions", "'Tell me about a time when...' ka jawab", [
    P("Interview mein poochha jata hai: 'Tell me about a time you faced a problem / disagreed / missed a deadline'. Is sawaal ka sabse accha format hai **STAR**: **S**ituation, **T**ask, **A**ction, **R**esult. Roughly 1 minute."),
    TBL(["Letter", "Matlab", "Ek line"], ["S — Situation", "Kahan / kab / kya hua", "In my college project, our deadline was in two days."], ["T — Task", "Aapki zimmedari", "I was responsible for the backend."], ["A — Action", "Aapne kya kiya (sabse lamba hissa)", "I split the work, fixed the main bugs first, and tested daily."], ["R — Result", "Nateeja + seekh", "We submitted on time and I learned to plan better."]),
    H("Poora example: 'Tell me about a time you handled pressure'"),
    EX("In my final-year project, our deadline was moved forward by a week. | final-year project में deadline एक हफ़्ता पहले हो गई।", "I was responsible for the backend, so I had to finish the API quickly. | मेरे ज़िम्मे backend था, इसलिए API जल्दी पूरी करनी थी।", "I listed the most important features, finished them first, and tested every day. | मैंने ज़रूरी features की सूची बनाई, पहले उन्हें पूरा किया और रोज़ test किया।", "As a result, we submitted on time, and our team got the highest marks in the class. | नतीजतन हमने समय पर जमा किया और class में सबसे ज़्यादा अंक मिले।", "I learned that clear priorities reduce stress. | मैंने सीखा कि साफ़ प्राथमिकताएँ तनाव घटाती हैं।"),
    TIP("**Action** mein 'I' bolo, 'we' nahi — interviewer aapka role jaanna chahta hai. 'We built...' ki jagah 'I wrote the API and my teammate designed the screens.'"),
    H("Useful starters"),
    EX("One example that comes to mind is ... | एक उदाहरण जो याद आता है ...", "The situation was that ... | स्थिति यह थी कि ...", "What I did was ... | मैंने यह किया कि ...", "The outcome was ... | नतीजा यह रहा ..."),
], [
    Q("STAR ka matlab?", ["Situation, Task, Action, Result", "Start, Test, Apply, Review", "Skill, Team, Aim, Reason", "Speak, Think, Answer, Repeat"], 0, "Situation, Task, Action, Result."),
    Q("Sabse zyada samay kis hisse ko dena chahiye?", ["Situation", "Action", "Task", "Thanks"], 1, "Action — aapne kya kiya."),
    Q("Action mein kya bolna chahiye?", ["'We' hi 'we'", "'I' — apna role", "Kuch nahi", "Doosre ka naam"], 1, "Apna role saaf batao: 'I'."),
    Q("Result mein kya hona chahiye?", ["Nateeja aur seekh", "Sirf complain", "Kuch nahi", "Naya sawaal"], 0, "Result + learning."),
    Q("Kaun sa STAR ka starter hai?", ["The situation was that ...", "Bye bye", "No idea", "Maybe"], 0, "Situation shuru karne ke liye."),
])

LESSON("p03", "s6", "Running a conversation: interrupt, clarify, summarise, close", "Baat shuru, beech mein aur khatm karna", [
    P("Fluent lagne ke liye grammar se zyada **conversation ke tools** kaam aate hain. In 4 skills ke ready phrases rakho."),
    H("1. Beech mein bolna (interrupt politely)"),
    EX("Sorry to interrupt, may I add something? | बीच में टोकने के लिए माफ़ कीजिए, क्या मैं कुछ जोड़ सकता हूँ?", "Could I just jump in here for a second? | क्या मैं यहाँ एक पल बोल सकता हूँ?", "Before we move on, I have one question. | आगे बढ़ने से पहले मेरा एक सवाल है।"),
    H("2. Samajh na aaye — clarify"),
    EX("Just to make sure I understand, you mean ...? | पक्का करने के लिए, आपका मतलब ... है?", "Could you give me an example? | क्या आप एक उदाहरण दे सकते हैं?", "When you say 'soon', what timeline do you have in mind? | 'जल्दी' से आपका क्या समय मतलब है?"),
    H("3. Summarise"),
    EX("So, to summarise, we agreed on three things. | तो संक्षेप में, हम तीन बातों पर सहमत हुए।", "Let me recap what I will do. | मैं दोहरा देता हूँ कि मैं क्या करूँगा।", "If I understood correctly, the next step is ... | अगर मैं सही समझा तो अगला क़दम ... है।"),
    H("4. Close the conversation"),
    EX("I do not want to take more of your time. Thank you. | मैं आपका और समय नहीं लूँगा। धन्यवाद।", "It was great speaking with you. Let us stay in touch. | आपसे बात करके अच्छा लगा। संपर्क में रहेंगे।", "I will send a follow-up email today. | मैं आज follow-up email भेजूँगा।"),
    TIP("**Chup rehne ki jagah** 'That is a good point' ya 'Interesting' bolo. Isse sochne ka time bhi milta hai aur conversation chalta rehta hai."),
], [
    Q("Beech mein polite tareeke se bolne ke liye?", ["Stop! Listen!", "Sorry to interrupt, may I add something?", "No no no.", "Wait."], 1, "Sorry to interrupt ..."),
    Q("Pakka karne ke liye kya bolte hain?", ["Just to make sure I understand, you mean ...?", "I do not care.", "Go on.", "Whatever."], 0, "Just to make sure I understand ..."),
    Q("Meeting ka saar kaise batate hain?", ["So, to summarise, ...", "Okay bye.", "Maybe.", "I think no."], 0, "To summarise ..."),
    Q("Baat khatam karne ka polite tareeka?", ["Bye.", "It was great speaking with you.", "Finish.", "Done."], 1, "It was great speaking with you."),
    Q("Sochne ka time lene ke liye?", ["That is a good point.", "Shut up.", "I sleep.", "No English."], 0, "'That is a good point' bolkar sochne ka time milta hai."),
])

LESSON("p04", "s6", "Say it better: common Indian-English phrases", "Office mein jo shabd alag lagte hain", [
    P("Kuch phrases India mein roz bole jaate hain par duniya ke baaki hisson mein confusing ya odd lagte hain. Ye galat nahi hain, par interview aur international teams mein **neutral English** better rehti hai."),
    TBL(["Aam Indian phrase", "Neutral English"],
        ["Do the needful.", "Please take the necessary action."],
        ["Kindly revert back.", "Please reply when you can."],
        ["Prepone the meeting.", "Move the meeting earlier."],
        ["I will do one thing.", "Here is what I will do."],
        ["Out of station.", "Out of town / travelling."],
        ["Passed out of college.", "Graduated from college."],
        ["What is your good name?", "What is your name?"],
        ["I have a doubt.", "I have a question."],
        ["Time pass.", "Just killing time."],
        ["Where are you staying?", "Where do you live?"]),
    EX("Could you move the meeting earlier, to ten? | क्या आप meeting को दस बजे तक पहले कर सकते हैं?", "I have a question about the second point. | दूसरे बिंदु पर मेरा एक सवाल है।", "I graduated in 2023. | मैंने 2023 में graduate किया।", "I will be out of town on Monday. | मैं सोमवार को शहर से बाहर रहूँगा।"),
    TIP("**'Doubt'** ka matlab English mein 'shak/ashanka' hota hai ('I doubt it'). Poochne ke liye **question** use karo."),
    H("Sentence patterns"),
    EX("I am working on it. | मैं इस पर काम कर रहा हूँ।", "I have been here for two years. | मैं यहाँ दो साल से हूँ।", "Can you help me with this? | क्या आप इसमें मेरी मदद कर सकते हैं?", "Let me check and get back to you. | मैं देखकर आपको बताता हूँ।"),
], [
    Q("'Prepone' ki jagah neutral?", ["Move it earlier", "Pre-do", "Before move", "Postpone"], 0, "Move it earlier."),
    Q("'Kindly revert back' ki jagah?", ["Please reply", "Go back", "Revert again", "Kindly back"], 0, "Please reply when you can."),
    Q("'I have a doubt' ki jagah?", ["I have a question", "I am doubt", "I doubt you", "I have doubting"], 0, "Question poochhne ke liye 'question'."),
    Q("'I passed out of college' ki jagah?", ["I graduated from college", "I passed away", "I out college", "I finished out"], 0, "Graduated."),
    Q("'Where are you staying?' ka matlab?", ["Aap kahan thehre hain (temporary)", "Aap kahan rehte hain", "Aap kahan hain", "Aap kaun hain"], 0, "Stay = thehrna. Permanent ghar ke liye 'live'."),
])

LESSON("p05", "s6", "Speaking with confidence: pace, pauses and stress", "Awaaz, ruk-ruk kar bolna aur zor dena", [
    P("Aapko English samajh aati hai par bolte waqt ruk jaate ho — iska ilaaj bahut shabd nahi, **bolne ki technique** hai."),
    H("1. Dheere bolo, ruko, saans lo"),
    P("Tez bolne se galtiyan badhti hain. **Dheere aur saaf** bolo. Comma ya full stop par ek chhota pause lo. Pause professional lagta hai — ghabrahat nahi."),
    EX("Good morning. (pause) I am Rahul. (pause) I am a Java developer. | नमस्ते। (रुकें) मैं राहुल हूँ। (रुकें) मैं Java developer हूँ।", "First, I fixed the bug. (pause) Then, I wrote a test. | पहले मैंने bug ठीक किया। (रुकें) फिर test लिखा।", "I have two options. (pause) The first one is simple. | मेरे पास दो विकल्प हैं। (रुकें) पहला आसान है।"),
    H("2. Key words par zor do (stress)"),
    P("Sentence ke important words (noun, verb, number, negative) par zor do; chhote words (a, the, to, of) halke bolo. Isse awaaz natural aur samajhne mein aasan lagti hai."),
    EX("I WANT to JOIN your COMpany. | मैं आपकी company join करना चाहता हूँ।", "The DEADline is FRIday, not MONday. | deadline शुक्रवार है, सोमवार नहीं।", "I did NOT say THAT. | मैंने वह नहीं कहा।"),
    H("3. Awaaz upar-neeche (intonation)"),
    P("**Yes/No sawaal** ke end mein awaaz upar jaati hai: 'Are you READY? ↗'. **Wh- sawaal aur statement** ke end mein neeche: 'Where do you LIVE? ↘' 'I live in DELhi. ↘'."),
    H("4. Filler words kam karo"),
    P("'Basically', 'actually', 'you know', 'like' bar-bar mat bolo. Rukna ho to **ek second chup raho** ya 'Let me think' bolo."),
    TIP("**Accent ki chinta mat karo.** Maksad *samjhe jaana* hai, native accent nahi. Dheere, saaf aur ruk-ruk kar bolne wala accent har office mein samjha jata hai."),
    H("Roz ka 5-minute exercise"),
    P("Koi bhi paragraph uthao, phone par record karo, sunke dekho: kahan jaldi ki, kahan 'umm' aaya. Dobara record karo. 3 baar mein farak sunai dega. Bolo section ke 'Check' button se bhi dekh sakte ho."),
], [
    Q("Confidence ke liye pehla step?", ["Tez bolna", "Dheere aur saaf bolna, pause lena", "Chup rehna", "Hindi mein bolna"], 1, "Dheere + pause."),
    Q("Stress kin words par dete hain?", ["Important words (noun, verb, number)", "Sirf 'the'", "Sab par barabar", "Kisi par nahi"], 0, "Key words par zor."),
    Q("Yes/No sawaal ke end mein awaaz?", ["Upar jaati hai", "Neeche jaati hai", "Wahi rehti hai", "Band"], 0, "Are you ready? ↗"),
    Q("Filler words ke bajay?", ["Ek second chup raho / Let me think", "Zor se bolo", "Hindi bolo", "Hans do"], 0, "Pause ya 'Let me think'."),
    Q("Accent ka asli maksad?", ["Native jaisa hona", "Samjhe jaana", "Tez bolna", "Gaana"], 1, "Understood hona sabse zaroori."),
])

LESSON("p06", "s6", "Numbers, dates, time, money and percentages", "Aankde bolna jo office mein roz aate hain", [
    P("Meeting aur interview mein numbers bolne mein aksar atakna padta hai. In patterns ko pakka karo."),
    TBL(["Likha hua", "Kaise bolte hain"], ["2:45", "two forty-five / a quarter to three"], ["9:30", "nine thirty / half past nine"], ["12.5%", "twelve point five percent"], ["1/2, 3/4", "a half, three quarters"], ["2024", "twenty twenty-four"], ["1,25,000", "one lakh twenty-five thousand (ya 125 thousand)"], ["₹50,000", "fifty thousand rupees"], ["15/08", "the fifteenth of August / August fifteenth"], ["3 > 2", "three is greater than two"], ["O(n)", "order of n"]),
    EX("The meeting starts at a quarter past ten. | meeting सवा दस बजे शुरू होती है।", "The salary is six lakh per annum. | वेतन छह लाख सालाना है।", "We improved performance by fifteen percent. | हमने प्रदर्शन पंद्रह प्रतिशत बेहतर किया।", "The deadline is on the twenty-first of March. | deadline इक्कीस मार्च है।", "It will take two to three weeks. | इसमें दो से तीन हफ़्ते लगेंगे।"),
    H("Approximate numbers"),
    EX("It takes about ten minutes. | इसमें लगभग दस मिनट लगते हैं।", "There are around fifty users. | लगभग पचास users हैं।", "Roughly two thousand records. | क़रीब दो हज़ार records।", "Nearly ninety percent of the tests passed. | लगभग नब्बे प्रतिशत tests पास हुए।"),
    TIP("Lakh/crore India ke bahar kam samjhe jaate hain. International call mein **thousand / million** use karo: 10 lakh = one million, 1 crore = ten million."),
], [
    Q("2:45 ko aur kaise bolte hain?", ["A quarter to three", "Half past two", "Two and nine", "Three forty"], 0, "Quarter to three."),
    Q("12.5% =", ["twelve point five percent", "twelve five percent", "one two five", "twelve and fifty"], 0, "point five."),
    Q("10 lakh =", ["one million", "ten thousand", "one billion", "one hundred thousand"], 0, "10 lakh = 1 million."),
    Q("'Lagbhag' ka English?", ["about / around / roughly", "never", "exactly", "only"], 0, "Approximate words."),
    Q("2024 ko bolte hain?", ["twenty twenty-four", "two zero two four", "two thousand and twenty four only", "twenty-twenty-four zero"], 0, "twenty twenty-four."),
])
