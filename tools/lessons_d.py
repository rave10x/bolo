LESSON("l20", "s3", "Introducing yourself", "Apne baare mein 8-10 sentence bolna", [
    P("Sabse pehli cheez jo har jagah kaam aati hai: **apna parichay**. Ek tayyar 8-10 sentence ka introduction rakho aur roz bolo. Neeche ek template hai — apni details se badal do."),
    TBL(["Kya batana hai", "Sentence"],
        ["Naam", "Hello, my name is ___."], ["Kahan se", "I am from ___. / I live in ___."], ["Umar", "I am ___ years old."], ["Kaam/padhai", "I am a student. / I am learning Java."],
        ["Parivaar", "I have ___ brothers and ___ sisters."], ["Shauk", "I like ___ and ___."], ["Lakshya", "I want to become a software developer."], ["Samapti", "Nice to meet you. / Thank you."]),
    H("Ek poora example"),
    EX("Hello, my name is Rahul. | नमस्ते, मेरा नाम राहुल है।", "I am from Uttar Pradesh. | मैं उत्तर प्रदेश से हूँ।", "I am twenty-five years old. | मैं पच्चीस साल का हूँ।", "I am learning Java and English. | मैं Java और अंग्रेज़ी सीख रहा हूँ।",
       "I studied in a government school. | मैंने सरकारी स्कूल में पढ़ाई की।", "I like cricket and music. | मुझे क्रिकेट और संगीत पसंद है।", "I want to become a software developer. | मैं सॉफ़्टवेयर डेवलपर बनना चाहता हूँ।", "Nice to meet you. | आपसे मिलकर अच्छा लगा।"),
    TIP("Bolo section mein 'Introducing Myself' pack se ye sentences record karo. Roz ek baar bolo — 2 hafte mein ye automatic ho jayega."),
    H("Jawab dene ke chhote tareeke"),
    EX("How are you? — I'm fine, thank you. And you? | आप कैसे हैं? — मैं ठीक हूँ, और आप?", "Where are you from? — I'm from Bihar. | आप कहाँ से हैं? — मैं बिहार से हूँ।", "What do you do? — I'm a student. | आप क्या करते हैं? — मैं विद्यार्थी हूँ।"),
], [
    Q("Apna naam batane ka sahi tareeka?", ["I am name Rahul.", "My name is Rahul.", "Me name Rahul.", "Name my Rahul."], 1, "My name is ..."),
    Q("'Main Bihar se hoon' =", ["I am from Bihar.", "I from Bihar.", "I am of Bihar.", "I is from Bihar."], 0, "I am from ..."),
    Q("'How are you?' ka natural jawab?", ["I am fine, thank you. And you?", "Fine you.", "I am good name.", "How are you."], 0, "Jawab + 'And you?'"),
    Q("'Mujhe cricket pasand hai' =", ["I like cricket.", "I am like cricket.", "I liking cricket.", "Me like cricket."], 0, "I like ..."),
    Q("'Aapse milkar achha laga' =", ["Nice to meet you.", "Nice meeting I.", "Good to see my.", "I nice you."], 0, "Nice to meet you."),
])

LESSON("l21", "s3", "Polite requests: can, could, would, may", "Tameez se maangna aur mana karna", [
    P("Kuch maangte waqt 'give me' bolna rude lagta hai. Polite sentence banane ke liye **Can / Could / Would / May** use karo + **please**."),
    TBL(["Phrase", "Kab", "Example"], ["Can you ...?", "Dosto/aam", "Can you help me?"], ["Could you ...?", "Zyada polite", "Could you open the window?"], ["Would you mind ...ing?", "Bahut polite", "Would you mind closing the door?"], ["May I ...?", "Ijazat maangna", "May I come in?"], ["Can I have ...?", "Kuch maangna", "Can I have some water?"]),
    EX("Could you please repeat that? | क्या आप कृपया वह दोहरा सकते हैं?", "May I use your pen? | क्या मैं आपकी कलम इस्तेमाल कर सकता हूँ?", "Can I have a glass of water? | क्या मुझे एक गिलास पानी मिल सकता है?", "Would you mind speaking slowly? | क्या आप धीरे बोलना पसंद करेंगे?"),
    H("Haan kehna"),
    EX("Sure! | ज़रूर!", "Of course. | बिल्कुल।", "No problem. | कोई बात नहीं।", "Yes, go ahead. | हाँ, कीजिए।"),
    H("Tameez se 'na' kehna"),
    EX("Sorry, I cannot. | माफ़ कीजिए, मैं नहीं कर सकता।", "I am afraid I am busy now. | मुझे डर है कि मैं अभी व्यस्त हूँ।", "Maybe later. | शायद बाद में।"),
    FIX("Give me water.", "Could I have some water, please?", "Request mein 'Could I have ... please' polite lagta hai."),
], [
    Q("Sabse polite kaun sa hai?", ["Give me a pen.", "Pen!", "Could you lend me a pen, please?", "I want a pen."], 2, "Could you ... please = polite."),
    Q("Ijazat maangne ke liye?", ["May I come in?", "I come in?", "Come in me?", "Do I come in?"], 0, "May I ...?"),
    Q("'Zaroor!' ka English?", ["Sure!", "Sorry!", "Maybe!", "Never!"], 0, "Sure = zaroor."),
    Q("'Would you mind' ke baad?", ["verb-ing", "to + verb", "verb-s", "verb-ed"], 0, "Would you mind closing...?"),
    Q("'Kya mujhe paani mil sakta hai?'", ["Can I have some water?", "Can I having water?", "I can have water?", "Do can I water?"], 0, "Can I have ...?"),
])

LESSON("l22", "s3", "Giving your opinion: I think, I agree, I disagree", "Apni raay rakhna", [
    P("Baat-cheet mein sirf 'yes/no' kaafi nahi — apni raay dena zaroori hai. Kuch ready phrases rakho."),
    TBL(["Kaam", "Phrase"], ["Raay dena", "I think ... / In my opinion, ... / I feel that ..."], ["Sehmat hona", "I agree. / You are right. / Exactly!"], ["Asehmat hona", "I do not agree. / I see it differently. / I am not sure about that."], ["Kaaran dena", "because ... / The reason is ... / For example, ..."], ["Sawaal ki raay", "What do you think? / What is your opinion?"]),
    EX("I think English is important for jobs. | मुझे लगता है नौकरी के लिए अंग्रेज़ी ज़रूरी है।", "I agree with you. | मैं आपसे सहमत हूँ।", "I do not agree because it is expensive. | मैं सहमत नहीं क्योंकि यह महँगा है।", "In my opinion, we should practice daily. | मेरी राय में हमें रोज़ अभ्यास करना चाहिए।", "What do you think? | आप क्या सोचते हैं?"),
    TIP("Raay dene ka formula: **Raay + because + ek example**. 'I like this app because it is simple. For example, I can listen and speak.'"),
    FIX("I am agree with you.", "I agree with you.", "'Agree' khud verb hai; am/is/are mat lagao."),
], [
    Q("'Main sehmat hoon' =", ["I agree.", "I am agree.", "I am agreeing.", "I agreed to."], 0, "I agree."),
    Q("Raay dene ka phrase?", ["I think ...", "I am think ...", "I thinking ...", "Think I ..."], 0, "I think ..."),
    Q("'Kyunki' ka English?", ["because", "but", "so", "then"], 0, "because."),
    Q("Asehmat hone ka polite tareeka?", ["You are wrong!", "I do not agree.", "No!", "Stop."], 1, "I do not agree."),
    Q("'Aap kya sochte hain?' =", ["What do you think?", "What you think?", "How you think?", "Why think you?"], 0, "What do you think?"),
])

LESSON("l23", "s3", "Connecting sentences: and, but, because, so, if, when", "Chhote sentences ko jodna", [
    P("Fluent lagne ke liye chhote-chhote sentences ko **jodne wale shabd** se jodo. Ye 6 shabd 80% kaam kar dete hain."),
    TBL(["Shabd", "Kaam", "Example"], ["and", "jodna", "I like tea and coffee."], ["but", "ulta/virodh", "I am tired but happy."], ["or", "vikalp", "Tea or coffee?"], ["because", "kaaran", "I stayed home because it rained."], ["so", "nateeja", "It rained, so I stayed home."], ["if", "sharat", "If you study, you will pass."], ["when", "jab", "When I was a child, I lived in a village."]),
    EX("I wanted to go, but I had no time. | मैं जाना चाहता था, लेकिन समय नहीं था।", "She is smart and hard-working. | वह होशियार और मेहनती है।", "I was late because the bus was late. | मैं देर से आया क्योंकि बस देर से थी।", "I was tired, so I slept early. | मैं थका था, इसलिए जल्दी सो गया।", "If it rains, we will stay at home. | अगर बारिश हुई, तो हम घर पर रहेंगे।"),
    TIP("**because** aur **so** dono ek sentence mein mat lagao: 'Because it rained, so I stayed' galat hai. Ya to 'Because it rained, I stayed' ya 'It rained, so I stayed'."),
    H("Order words"),
    EX("First, I wake up. Then I drink tea. | पहले मैं उठता हूँ। फिर चाय पीता हूँ।", "After that, I go to college. | उसके बाद मैं कॉलेज जाता हूँ।", "Finally, I come home. | आख़िर में मैं घर आता हूँ।"),
    FIX("Because I was tired, so I slept.", "Because I was tired, I slept.", "because और so ek saath nahi aate."),
], [
    Q("I was tired ___ I slept early.", ["because", "so", "but", "if"], 1, "Nateeja: so."),
    Q("I slept early ___ I was tired.", ["because", "so", "but", "or"], 0, "Kaaran: because."),
    Q("I like tea ___ I do not like coffee.", ["and", "but", "so", "because"], 1, "Virodh: but."),
    Q("___ you study, you will pass.", ["If", "But", "So", "Or"], 0, "Sharat: If."),
    Q("Sahi sentence?", ["Because it rained, so I stayed.", "Because it rained, I stayed.", "It rained because so I stayed.", "So because I stayed."], 1, "Ek hi connector."),
])

LESSON("l24", "s4", "Interview: 'Tell me about yourself'", "Job interview ka sabse pehla sawaal", [
    P("Har interview ka pehla sawaal: **'Tell me about yourself.'** Iska jawab 45-60 second ka hona chahiye: **Present → Past → Future**. Ek tayyar jawab rakho aur roz bolo."),
    TBL(["Hissa", "Kya bolna hai", "Example"], ["Present", "Abhi kya kar rahe ho", "I am a Java developer in training."], ["Past", "Padhai / experience / project", "I built a small expense app and learned data structures."], ["Skill", "2-3 ke naam", "I know Java, SQL and basic Git."], ["Future", "Aap kya chahte ho", "I want to grow as a backend developer in your company."]),
    H("Poora jawab (copy karke apna banao)"),
    EX("Thank you for this opportunity. | इस अवसर के लिए धन्यवाद।", "My name is Rahul. I am learning Java and data structures. | मेरा नाम राहुल है। मैं Java और data structures सीख रहा हूँ।",
       "I built a small project to track expenses. | मैंने ख़र्चे ट्रैक करने के लिए एक छोटा प्रोजेक्ट बनाया।", "I enjoy solving problems and I practice every day. | मुझे समस्याएँ हल करना पसंद है और मैं रोज़ अभ्यास करता हूँ।",
       "I want to work in a team and learn from experienced people. | मैं टीम में काम करना और अनुभवी लोगों से सीखना चाहता हूँ।"),
    H("Aur common sawaal"),
    EX("What are your strengths? — I am a fast learner and I work hard. | आपकी ताक़त क्या है?", "Why should we hire you? — Because I learn quickly and I am serious about my work. | हम आपको क्यों रखें?", "Do you have any questions? — Yes, what will my first project be? | क्या आपका कोई सवाल है?"),
    TIP("Jawab **seedhe sach** do. 'I do not know this yet, but I can learn it quickly' bolna bilkul theek hai — jhooth se behtar."),
], [
    Q("'Tell me about yourself' ka jawab kitne second ka?", ["5 second", "45-60 second", "10 minute", "Kuch nahi bolna"], 1, "Chhota aur clear: 45-60 second."),
    Q("Jawab ka sahi order?", ["Present → Past → Future", "Future → Past", "Sirf Past", "Sirf Hobby"], 0, "Present, past, future."),
    Q("Jab jawab na pata ho?", ["Jhooth bolo", "I do not know this yet, but I can learn it.", "Chup raho", "Phone dekho"], 1, "Sach + seekhne ki taiyari."),
    Q("'Main jaldi seekhta hoon' =", ["I am a fast learner.", "I learn fastly.", "I am fast learn.", "I fast learner."], 0, "I am a fast learner."),
    Q("Interview ke end mein kya bolo?", ["Thank you for your time.", "Bye.", "Ok.", "Hmm."], 0, "Shukriya bolna acchi aadat hai."),
])

LESSON("l25", "s4", "Daily stand-up update", "Team ko roz ka update dena", [
    P("Software teams mein roz **stand-up** hota hai. 3 cheezein batani hoti hain: **Kal kya kiya, aaj kya karunga, koi rukawat (blocker)**."),
    TBL(["Hissa", "Template"], ["Yesterday", "Yesterday I worked on / finished / fixed ___."], ["Today", "Today I will work on / start / complete ___."], ["Blockers", "I am blocked on ___. / I have no blockers."]),
    EX("Yesterday I fixed a login bug. | कल मैंने एक login bug ठीक किया।", "Today I will write tests for the API. | आज मैं API के tests लिखूँगा।", "I am blocked on the database access. | मैं database access की वजह से रुका हूँ।", "I have no blockers. | मेरी कोई रुकावट नहीं है।"),
    H("Kaam ke status ke liye phrases"),
    EX("It is almost done. | यह लगभग हो गया है।", "I am still working on it. | मैं अभी भी इस पर काम कर रहा हूँ।", "I need more time. | मुझे और समय चाहिए।", "It will be ready by Friday. | यह शुक्रवार तक तैयार होगा।", "Can you help me with this? | क्या आप इसमें मेरी मदद कर सकते हैं?"),
    TIP("Past ke liye **-ed / V2** (fixed, worked), future ke liye **will**. Ye lesson L16 aur L17 ka asli use hai."),
], [
    Q("'Kal maine bug fix kiya' =", ["Yesterday I fixed a bug.", "Yesterday I fix a bug.", "Yesterday I will fix a bug.", "Yesterday I am fixing a bug."], 0, "Past: fixed."),
    Q("'Aaj main test likhunga' =", ["Today I will write tests.", "Today I wrote tests.", "Today I writing tests.", "Today I have wrote tests."], 0, "Future: will write."),
    Q("Blocker ka matlab?", ["Rukawat", "Khel", "Photo", "Chhutti"], 0, "Blocker = jo kaam rok de."),
    Q("'Mujhe aur samay chahiye' =", ["I need more time.", "I need more times.", "I am need time.", "I needs time."], 0, "I need more time."),
    Q("'Yeh lagbhag ho gaya hai' =", ["It is almost done.", "It almost done is.", "It is done almost not.", "Almost it done."], 0, "It is almost done."),
])

LESSON("l26", "s4", "Writing emails and chat messages", "Professional likhna", [
    P("Email chhoti, saaf aur polite honi chahiye. Standard structure: **Subject → Greeting → Kaaran → Request → Closing**."),
    TBL(["Hissa", "Example"], ["Subject", "Leave request for Friday"], ["Greeting", "Hi Mr. Sharma, / Dear Sir,"], ["Kaaran", "I am writing to ask for a day off on Friday."], ["Request/Detail", "I will finish my work before I leave."], ["Closing", "Thank you for your time. / Best regards, Rahul"]),
    EX("I am writing to ask about the project status. | मैं प्रोजेक्ट की स्थिति के बारे में पूछने के लिए लिख रहा हूँ।", "Please find the report attached. | कृपया संलग्न रिपोर्ट देखिए।", "Could you please send me the details? | क्या आप कृपया मुझे विवरण भेज सकते हैं?", "I will get back to you by tomorrow. | मैं कल तक आपसे संपर्क करूँगा।", "Thank you for your help. | आपकी मदद के लिए धन्यवाद।"),
    H("Chat (Slack/WhatsApp) mein"),
    EX("Sure, I will check. | ज़रूर, मैं देखता हूँ।", "Thanks, got it. | धन्यवाद, समझ गया।", "Sorry, I missed your message. | माफ़ कीजिए, आपका संदेश छूट गया।", "Can we talk for five minutes? | क्या हम पाँच मिनट बात कर सकते हैं?"),
    TIP("'Do the needful' aur 'kindly revert' ki jagah seedhe likho: 'Please take action' / 'Please reply'. Saaf aur seedhi English zyada professional lagti hai."),
    P("Closing mein: **Best regards,** ya **Thanks,** phir apna naam."),
], [
    Q("Email ka pehla hissa?", ["Subject", "Closing", "Signature", "Attachment"], 0, "Subject line pehle."),
    Q("'Please reply' ki jagah purana tareeka?", ["Kindly revert", "Please respond", "Please answer", "Reply me"], 0, "'Kindly revert' purani Indian English hai."),
    Q("Closing ke liye sahi?", ["Best regards, Rahul", "Bye bye Rahul", "From Rahul", "Rahul is me"], 0, "Best regards, Name."),
    Q("'Main jaldi reply karunga' =", ["I will get back to you soon.", "I revert you soon.", "I will reverting.", "I back you soon."], 0, "get back to you."),
    Q("'Please find attached' ka matlab?", ["Sanlagn dekhiye", "Dhundhiye", "Wapas aaiye", "Phone kijiye"], 0, "Attached file ke liye."),
])

LESSON("l27", "s4", "Explaining a bug or code in English", "Code samjhana aur bug batana", [
    P("Ek developer ka kaam sirf code likhna nahi — **samjhana** bhi hai. Ye phrases team meeting aur interview dono mein kaam aate hain."),
    H("Bug batana"),
    EX("I found a bug in the login page. | मुझे login page में एक bug मिला।", "The app crashes when I click the button. | बटन दबाने पर ऐप क्रैश हो जाता है।", "I get a null pointer error here. | मुझे यहाँ null pointer error आता है।", "It works on my machine, but not on the server. | यह मेरी मशीन पर चलता है, पर server पर नहीं।"),
    H("Kya kiya ye batana"),
    EX("I fixed it by adding a null check. | मैंने null check जोड़कर इसे ठीक किया।", "I changed the loop to run only once. | मैंने loop को सिर्फ़ एक बार चलाने के लिए बदला।", "I added a test to avoid this in future. | मैंने भविष्य में इससे बचने के लिए एक test जोड़ा।"),
    H("Code samjhana"),
    EX("This function takes two numbers and returns their sum. | यह function दो संख्याएँ लेता है और उनका जोड़ लौटाता है।", "First, we read the input. Then we sort the array. | पहले हम input पढ़ते हैं। फिर array sort करते हैं।", "The time complexity is order of n. | समय जटिलता O(n) है।", "I used a hash map to make it faster. | मैंने इसे तेज़ बनाने के लिए hash map इस्तेमाल किया।"),
    TIP("Explain karne ka simple formula: **Problem → Idea → Steps → Result.** 'The problem is ___. My idea is ___. First ___, then ___. So the result is ___.'"),
    H("Samajh na aaye to"),
    EX("Could you explain that again? | क्या आप वह फिर से समझा सकते हैं?", "Sorry, I did not get it. | माफ़ कीजिए, मैं समझा नहीं।", "Do you mean this function? | क्या आपका मतलब यह function है?"),
], [
    Q("'App crash ho jata hai' =", ["The app crashes.", "The app crash.", "The app is crashing it.", "The app crashed it."], 0, "The app crashes."),
    Q("'Maine ise fix kiya' =", ["I fixed it.", "I fix it yesterday.", "I fixing it.", "I have fixes it."], 0, "Past: fixed."),
    Q("Explain karne ka sahi order?", ["Problem, idea, steps, result", "Result, steps, idea", "Sirf code", "Sirf result"], 0, "Problem → Idea → Steps → Result."),
    Q("'Mujhe samajh nahi aaya' =", ["I did not get it.", "I not understand it.", "I am not got.", "I did not gets."], 0, "I did not get it."),
    Q("'O(n)' ko bolte hain?", ["order of n", "o of nothing", "oh en", "on"], 0, "'order of n' ya 'linear time'."),
])

LESSON("l28", "s5", "Think in English: daily practice plan", "Roz 30 minute ka fluency plan", [
    P("Bahut saari grammar jaanne se English nahi aati — **roz thoda bolne** se aati hai. Ye plan Bolo app ke saath chalao. Koi shortcut nahi hai, par ye tareeka sabse tez hai."),
    TBL(["Samay", "Kya karna hai", "Bolo mein kahan"], ["5 min", "5 naye shabd seekho", "Shabd → Learn"], ["5 min", "Purane shabd review", "Shabd → Review"], ["10 min", "1 lesson padho + test", "Seekho"], ["10 min", "5 sentences bolo aur record karo", "Bolo"], ["5 min", "Mixed test", "Abhyas"]),
    H("5 aadatein jo sabse zyada kaam karti hain"),
    P("**1. Shadowing:** koi sentence suno, turant saath mein bolo. Phir record karo aur compare karo.\n**2. Khud se baat karo:** din bhar jo kar rahe ho, English mein bolo. 'I am making tea. I am pouring water.'\n**3. Diary:** roz 3 sentence likho — aaj kya kiya. Galti ki chinta mat karo.\n**4. English mein sochna:** Hindi se translate mat karo, seedhe chhote sentence socho.\n**5. Roz karo:** 30 minute roz, 3 ghante haftey mein ek baar se behtar hai."),
    EX("I am making tea. | मैं चाय बना रहा हूँ।", "Today I learned five new words. | आज मैंने पाँच नए शब्द सीखे।", "I practised speaking for twenty minutes. | मैंने बीस मिनट बोलने का अभ्यास किया।"),
    TIP("**Galti hona normal hai.** Bolte waqt grammar perfect ho ye zaroori nahi — saamne wala samajh jaye, itna kaafi hai. Pehle fluency, phir accuracy."),
], [
    Q("Shadowing kya hai?", ["Sunkar saath mein bolna", "Sirf padhna", "Likhna", "Sona"], 0, "Suno aur saath mein bolo."),
    Q("Kya behtar hai?", ["30 minute roz", "3 ghante ek baar", "Kabhi nahi", "Sirf exam se pehle"], 0, "Roz thoda — consistency."),
    Q("Diary mein kya likhna chahiye?", ["Aaj kya kiya (3 sentence)", "Sirf Hindi", "Kuch nahi", "Copy"], 0, "Chhote sentence, roz."),
    Q("Bolte waqt sabse pehle kya zaroori?", ["Perfect grammar", "Saamne wala samajh jaye", "Tez bolna", "Zor se bolna"], 1, "Pehle samjhana, phir perfect karna."),
    Q("Khud se English mein baat karne ka fayda?", ["Bolne ki aadat", "Kuch nahi", "Awaaz kharab", "Time waste"], 0, "Practice ka sabse aasan tareeka."),
])

LESSON("l29", "s5", "Buying time and keeping the conversation going", "Jab words na milein", [
    P("Bolte waqt rukna normal hai. Chup rehne ke bajay **filler phrases** bolo — isse sochne ka samay mil jata hai aur tum confident lagte ho."),
    H("Sochne ka samay lene ke liye"),
    EX("Let me think. | मुझे सोचने दीजिए।", "Well, ... | अच्छा, ...", "That is a good question. | यह एक अच्छा सवाल है।", "How can I say this? | मैं यह कैसे कहूँ?", "Just a moment, please. | बस एक पल, कृपया।"),
    H("Jab word yaad na aaye"),
    EX("What is the word for ... in English? | ... को अंग्रेज़ी में क्या कहते हैं?", "I mean, ... | मेरा मतलब है, ...", "It is something like ... | यह कुछ ऐसा है जैसे ...", "Sorry, my English is not perfect. | माफ़ कीजिए, मेरी अंग्रेज़ी perfect नहीं है।"),
    H("Samajh na aaye to"),
    EX("Could you say that again, please? | क्या आप कृपया दोबारा कह सकते हैं?", "Could you speak more slowly? | क्या आप थोड़ा धीरे बोल सकते हैं?", "What do you mean? | आपका क्या मतलब है?", "Do you mean ...? | क्या आपका मतलब ... है?"),
    H("Baat aage badhane ke liye"),
    EX("Really? That is interesting! | सच में? यह दिलचस्प है!", "And what about you? | और आप?", "Tell me more. | और बताइए।", "I see. | अच्छा, समझा।"),
    TIP("Ek English sentence na aaye to **chhota sentence** bolo. 'It is very good' ke bajay lamba nahi soch sakte to 'Good!' bhi chalega."),
], [
    Q("Sochne ka samay lene ke liye?", ["Let me think.", "I die.", "Go away.", "No English."], 0, "Let me think."),
    Q("Word yaad na aaye to?", ["Chup ho jao", "What is the word for ... in English?", "Hindi mein bolo", "Phone rakh do"], 1, "Seedha poochh lo."),
    Q("'Dobara boliye' ka polite tareeka?", ["Say again!", "Could you say that again, please?", "What?", "Repeat repeat."], 1, "Could you ... please."),
    Q("'Aur bataiye' =", ["Tell me more.", "Tell more me.", "More tell.", "Say me more."], 0, "Tell me more."),
    Q("Baat aage badhane ke liye?", ["And what about you?", "Stop.", "Nothing.", "Bye."], 0, "Sawaal wapas poochho."),
])

LESSON("l30", "s5", "Understanding fast English: contractions and linking", "Log tez kyon bolte hain, samajh kaise aaye", [
    P("Jab log English tez bolte hain to sunne mein shabd mile-jule lagte hain. Iska raaz: **contractions** (chhote roop) aur **linking** (shabd jud jaate hain). Ye jaan lo to sunna aasan ho jata hai."),
    H("Contractions"),
    TBL(["Likha hua", "Bola hua", "Matlab"], ["I am", "I'm", "main hoon"], ["do not", "don't", "nahi karte"], ["cannot", "can't", "nahi sakta"], ["will not", "won't", "nahi karunga"], ["I will", "I'll", "main karunga"], ["it is", "it's", "yah hai"], ["I have", "I've", "mere paas hai"]),
    EX("I don't know. | मैं नहीं जानता।", "She can't come today. | वह आज नहीं आ सकती।", "I'll call you later. | मैं बाद में फ़ोन करूँगा।", "It's very late. | बहुत देर हो गई है।"),
    H("Linking: shabd jud jaate hain"),
    TBL(["Likha", "Aisa sunai deta hai"], ["What do you want?", "Whaddaya want?"], ["Give me", "Gimme"], ["going to", "gonna"], ["want to", "wanna"], ["a lot of", "a lotta"], ["kind of", "kinda"]),
    TIP("Ye informal bolchaal hai. Likhne mein 'going to', 'want to' hi likho. Par jab koi 'gonna' ya 'wanna' bole to samajh jao ki wo kya keh raha hai."),
    H("Practice kaise karein"),
    P("Koi bhi chhoti video ya podcast lagao, **subtitles ke saath** ek sentence suno aur dobara 5 baar suno. Phir bina subtitles ke suno. Roz 10 minute."),
    EX("I'm going to go now. | मैं अब जाने वाला हूँ।", "I don't want to be late. | मैं देर नहीं करना चाहता।", "Can you give me a minute? | क्या आप मुझे एक मिनट दे सकते हैं?"),
], [
    Q("'don't' ka poora roop?", ["do not", "did not", "does not", "dont"], 0, "don't = do not."),
    Q("'gonna' ka matlab?", ["going to", "gone", "gotten", "go on"], 0, "gonna = going to."),
    Q("'I'll' ka matlab?", ["I will", "I all", "I ill", "I eel"], 0, "I'll = I will."),
    Q("Tez English samajhne ke liye kya karein?", ["Subtitles ke saath baar-baar sunna", "Sirf padhna", "Chhod dena", "Ek baar sunna"], 0, "Sunna + repeat."),
    Q("'can't' ka matlab?", ["nahi sakta", "kar sakta hai", "karna chahiye", "kal"], 0, "can't = cannot."),
])
