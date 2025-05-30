pub mod argparse {
    use std::collections::HashMap;
    use std::process::exit;
    use itertools::Itertools;

    pub struct Argument {
        pub trigger: &'static str,
        pub mtrigger: &'static str,
        pub isflag: bool,
        pub isrequired: bool,
        pub help: &'static str
    }

    pub struct ArgumentParser { 
        pub argsvec: Vec<String>,
        pub prefix: char,
        pub name: &'static str,
        pub desc: &'static str,
        pub args: &'static [Argument]
    }

    impl ArgumentParser {
        pub fn compile(&self) -> FinalizedArguments {
            // check for predefined help and add if doesn't exist
            let mut newargs: Vec<&Argument> = vec![];
            let help_arg = &Argument { trigger: "help", mtrigger: "h", isflag: true, isrequired: false, help: "sends help" };
            newargs.push(help_arg);

            // panicking when help or h is used as a trigger or mini trigger so ppl know why its failing
            for arg in self.args {
                if arg.trigger == help_arg.trigger || arg.trigger == help_arg.mtrigger { panic!("{}", format!("{} uses {} as trigger, which is reserved (you cannot use help / h for any trigger or mini trigger)", arg.trigger, help_arg.trigger)) }
                else if arg.mtrigger == help_arg.trigger || arg.mtrigger == help_arg.mtrigger { panic!("{}", format!("{} uses {} as mtrigger, which is reserved (you cannot use help / h for any trigger or mini trigger)", arg.trigger, help_arg.trigger)) }
                newargs.push(arg);
            }

            // checks for redefinitions of programmers arguments' triggers or mini triggers
            let mut fargs: Vec<FinalizedArgument> = vec![];
            let mut reading: bool = false;
            let mut current: &'static Argument = &Argument { trigger: "", mtrigger: "", isflag: false, isrequired: false, help: "" };
            let mut current_val: String = "".to_string();
            let mut i: usize = 0;

            let mut _trigs: Vec<String> = vec![];
            let mut _mtrigs: Vec<String> = vec![];
            for arg in self.args {
                if _trigs.contains(&arg.trigger.to_string()) || _trigs.contains(&arg.mtrigger.to_string()) { panic!("{}", format!("FATAL redefinition of '{}' / '{}'", arg.trigger, arg.mtrigger)) }
                if _mtrigs.contains(&arg.mtrigger.to_string()) || _mtrigs.contains(&arg.trigger.to_string()) { panic!("{}", format!("FATAL redefinition of '{}' / '{}'", arg.trigger, arg.mtrigger)) }
                _trigs.push(arg.trigger.to_string());
                _mtrigs.push(arg.mtrigger.to_string());
            }

            for a in self.get_args() {
                i += 1;
                // set astr to the current argument without the prefix(s)
                let aspl: Vec<_> = a.split(self.prefix).collect();
                let astr: &str = aspl[aspl.len()-1];

                if reading {
                    if reading && a.starts_with(self.prefix) { 
                        // starts redefinition of new argument after encountering prefix during reading
                        if current_val.is_empty() { panic!("{}", format!("cannot define new variable '{}' before defining the variable before '{}'", a, current.trigger)); }
                        fargs.push(FinalizedArgument { arg: current, val: current_val.clone().trim().to_string(), flagged: false });
                        current = &Argument { trigger: "", mtrigger: "", isflag: false, isrequired: false, help: "" };
                        if current_val.is_empty() { panic!("FATAL attempting to add null value for '{}'", a); }

                        current_val = "".to_string();
                        reading = false;

                        let mut found = false;
                        for arg in newargs.clone() {
                            if (arg.trigger == astr && aspl.len() == 3) || (arg.mtrigger == astr && aspl.len() == 2) {
                                current = arg;
                                found = true;

                                if arg.isflag { fargs.push(FinalizedArgument { arg: current, val: "".to_string(), flagged: true }); } 
                                else { reading = true; }
                                break;
                            }
                        }
                        if !found { panic!("FATAL trigger '{}' not found", a); }
                    } else { current_val = format!("{}{} ", current_val, astr);}
                } else {
                    if !a.starts_with(self.prefix) { continue; }
                    let mut found = false;
                    for arg in self.args {
                        if (arg.trigger == astr && aspl.len() == 3) || (arg.mtrigger == astr && aspl.len() == 2) {
                            current = arg;
                            found = true;
                            
                            if arg.isflag { fargs.push(FinalizedArgument { arg: current, val: "".to_string(), flagged: true }); } 
                            else { reading = true; }
                            break;
                        }
                    }
                    if !found { panic!("FATAL trigger '{}' not found", a); }
                }

                if i == self.get_args().len() {
                    if current_val.is_empty() && !current.isflag { panic!("attempting to add null value for '{}'", a); }
                    else if !current.isflag { fargs.push(FinalizedArgument { arg: current, val: current_val.clone().trim().to_string(), flagged: false }); }
                }
            }

            // checks for redefinitions of users arguments' triggers or mini triggers
            let mut _utrigs: Vec<String> = vec![];
            let mut _umtrigs: Vec<String> = vec![];
            for arg in fargs.iter().clone() {
                if _utrigs.contains(&arg.arg.trigger.to_string()) { panic!("{}", format!("FATAL redefinition of '{}' / '{}' in command line", arg.arg.trigger, arg.arg.mtrigger)) }
                if _umtrigs.contains(&arg.arg.mtrigger.to_string()) { panic!("{}", format!("FATAL redefinition of '{}' / '{}' in command line", arg.arg.trigger, arg.arg.mtrigger)) }
                _utrigs.push(arg.arg.trigger.to_string());
                _umtrigs.push(arg.arg.mtrigger.to_string());
            }

            // checks for missing required arguments and adds unflagged arguments as false
            let mut missing: Vec<&str> = vec![];
            for a in self.args {
                let mut found = false;
                for fa in fargs.iter().clone() {
                    if fa.arg.trigger == a.trigger && fa.arg.mtrigger == a.mtrigger { 
                        found = true;
                        break; 
                    }
                }

                if !found {
                    if a.isflag { fargs.push(FinalizedArgument { arg: a, val: "".to_string(), flagged: false }); }
                    if !a.isrequired { continue; }
                    missing.push(a.trigger);
                }
            }

            if missing.len() > 0 { panic!("FATAL missing required argument(s): {}", missing.join(", ")) }

            // checks if someone needs help
            let finalargs = FinalizedArguments { args: fargs };
            match finalargs.get("help") {
                Some(arg) => {
                    if arg.flagged {
                        println!("{}", self.name);
                        println!("{}\n", self.desc);
                        println!("arguments:");

                        let mut stuff: HashMap<&str, (String, &str)> = HashMap::new();
                        let mut longest_trig = 0;
                        for arg in newargs {
                            let trig = self.prefix.to_string() + &self.prefix.to_string() + &arg.trigger.to_string() + ", " + &self.prefix.to_string() + &arg.mtrigger.to_string();
                            let triglen = trig.len();
                            if triglen > longest_trig { longest_trig = triglen }
                            stuff.insert(arg.trigger, (trig, arg.help));
                        }

                        for (_, stuff) in stuff.iter().sorted() {
                            println!("{}", format!("{}{}{}{}", (0..2).map(|_| " ").collect::<String>(), stuff.0, (0..((longest_trig - stuff.0.len())+4)).map(|_| " ").collect::<String>(), stuff.1));
                        }

                        exit(0);
                    }
                }
                None => {}
            }

            return finalargs;
        }

        fn get_args(&self) -> Vec<String> {
            let mut l = self.argsvec.clone();
            l.remove(0);
            return l;
        }
    }

    pub struct FinalizedArgument {
        pub arg: &'static Argument,
        pub val: String,
        pub flagged: bool
    }

    pub struct FinalizedArguments {
        pub args: Vec<FinalizedArgument>
    }

    impl FinalizedArguments {
        pub fn get(&self, trig: &str) -> Option<&FinalizedArgument> {
            for a in self.args.iter().clone() {
                if a.arg.trigger == trig { return Some(a); }
            }
            
            return None;
        }
    }
}