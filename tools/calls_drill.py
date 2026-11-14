# Call-interview response drill: pick the best thing to say. ITEMS is filled via Q(...).
def add(q, opts, ans, why): ITEMS.append(Q(q, opts, ans, why))

add(T("Interviewer: 'Hello Rahul, is this a good time to talk?' Aap taiyar ho. Best jawab?", "Interviewer: 'Hello Rahul, is this a good time to talk?' You are ready. Best reply?"),
    ["Yes, this is a good time. Thank you for calling.", "Hmm, say fast.", "What do you want?", "I am eating, call later."], 0, T("Haan + dhanyavaad.", "Yes + thanks."))
add(T("Interviewer: 'Can you tell me about yourself?' Sabse accha shuruaat?", "Interviewer: 'Can you tell me about yourself?' Best opening?"),
    ["My name is Rahul, age 22, unmarried.", "I am a Java developer with a year of project experience, and I recently built an expense tracker.", "You can read my resume.", "Nothing special."], 1, T("Present, kaam, aur ek project — seedha aur relevant.", "Present, work and one project — direct and relevant."))
add(T("Aapko sawaal samajh nahi aaya. Best response?", "You did not understand the question. Best response?"),
    ["What?", "Sorry, could you rephrase the question, please?", "Skip it.", "I don't know."], 1, T("Polite tareeke se dobara poochho.", "Politely ask again."))
add(T("Awaaz toot rahi hai. Kya bolein?", "The audio is breaking up. What do you say?"),
    ["You are breaking up a little. Could you repeat the last part?", "Your line is bad!", "Do something.", "Say nothing and wait."], 0, T("Dikkat batao aur polite request karo.", "State the problem and ask politely."))
add(T("Interviewer: 'Why do you want to leave your current job?' Best jawab?", "Interviewer: 'Why do you want to leave your current job?' Best reply?"),
    ["My manager is bad and the pay is low.", "I have learned a lot and now I am looking for bigger responsibilities and a stronger team.", "Because everybody leaves.", "I just want more holidays."], 1, T("Burai nahi; seekhna aur growth.", "No criticism; learning and growth."))
add(T("Interviewer: 'What is your expected salary?' Best jawab?", "Interviewer: 'What is your expected salary?' Best reply?"),
    ["I will not tell.", "Whatever you give.", "I am looking for something in the range of X to Y, depending on the role.", "One crore."], 2, T("Range + role par depend.", "A range + depends on the role."))
add(T("Interviewer: 'When can you join?' (notice period 30 din)", "Interviewer: 'When can you join?' (notice period 30 days)"),
    ["My notice period is 30 days, and I can discuss an earlier date if needed.", "After some time, maybe.", "Never.", "I don't know my notice period."], 0, T("Saaf date aur flexibility.", "A clear date plus flexibility."))
add(T("Interviewer: 'Do you have any questions for us?' Best jawab?", "Interviewer: 'Do you have any questions for us?' Best reply?"),
    ["No.", "Yes, what does a typical day look like for this role?", "Can I leave early on Fridays?", "When will you pay me?"], 1, T("Role ke baare mein ek achha sawaal.", "A good question about the role."))
add(T("Aapko jawab nahi pata. Best response?", "You do not know the answer. Best response?"),
    ["I have not worked with that directly, but I understand the idea and I can learn it quickly.", "That is a silly question.", "Yes, I know everything.", "Let us change the topic."], 0, T("Sach + seekhne ki taiyari.", "Honesty + willingness to learn."))
add(T("Interviewer: 'Tell me about a time you handled pressure.' Pehla kadam?", "Interviewer: 'Tell me about a time you handled pressure.' First step?"),
    ["Start with the Situation: where, when, what happened.", "Say 'I never face pressure.'", "Say your goals for 2030.", "Say 'We did it.'"], 0, T("STAR: Situation se shuru.", "STAR: start with the Situation."))
add(T("Call kat gayi aur aapne wapas judkar kya kaha?", "The call dropped and you reconnected. What do you say?"),
    ["Sorry about that, my connection dropped. Where were we?", "Your company has bad internet.", "Why did you hang up?", "Hello hello hello hello."], 0, T("Maafi + wapas shuru karo.", "Apologise + continue."))
add(T("Aapko sochne ka time chahiye. Kya bolein?", "You need a moment to think. What do you say?"),
    ["Let me think about that for a moment.", "Umm... umm... umm...", "Next question please.", "Ask something else."], 0, T("Ye phrase 5 second ka time deta hai.", "This phrase buys you 5 seconds."))
add(T("Interviewer: 'Why should we hire you?' Best structure?", "Interviewer: 'Why should we hire you?' Best structure?"),
    ["Because I need a job.", "My skills match your needs, I learn fast, and I have built relevant projects.", "Because I am the best in the world.", "I do not know."], 1, T("Skills + seekhna + proof.", "Skills + learning + proof."))
add(T("Call ke end mein sabse acha closing?", "The best closing at the end of the call?"),
    ["Okay bye.", "Thank you for your time. I am very interested in this role. What are the next steps?", "So, will you select me?", "Do not call me again."], 1, T("Dhanyavaad + interest + agla step.", "Thanks + interest + next step."))
add(T("Interviewer ne English bahut tez boli. Kya bolein?", "The interviewer speaks very fast. What do you say?"),
    ["Could you speak a little slower, please?", "Speak English properly!", "I cannot understand you, bye.", "Nod and say nothing."], 0, T("Polite request: thoda dheere.", "A polite request: a bit slower."))
add(T("Aap apna role batana chahte ho. Kaun sa sentence behtar?", "You want to state your own role. Which sentence is better?"),
    ["We built the project.", "I wrote the API and my teammate designed the screens.", "The project got built.", "Everyone did something."], 1, T("'I' se apna role saaf hota hai.", "'I' makes your role clear."))
add(T("Video call mein koi aapko screen share karne ko kehta hai. Kya bolein?", "On a video call you are asked to share your screen. What do you say?"),
    ["Sure, let me share my screen. Can you see it?", "I do not know how.", "No, I do not share.", "Wait one hour."], 0, T("Karo aur confirm karo.", "Do it and confirm."))
