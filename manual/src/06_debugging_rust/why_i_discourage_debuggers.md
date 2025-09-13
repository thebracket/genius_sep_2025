# Why I Discourage Debuggers

Bill Kennedy (my boss at Ardan, and a Go guru) tells people not to use debuggers. I don't go that far, but I do think that debuggers can be overused. Usually, a debugger means you don't really understand what's happening. That's often a sign that the *code is too complicated*, more than a sign that you need a debugger.

I'm not going to tell you never to use a debugger, but I will say that I've maybe used a debugger twice shipping LibreQoS - 50k lines of Rust, kernel C and Python code. There's tracing all over the place, but very little use of an actual debugger!