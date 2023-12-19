use std::{fs::File, io::Read, mem::replace};

struct Gear {
    cool_looking:u32,
    musical:u32,
    aerodynamic:u32,
    shiny:u32,
}

impl Gear {
    fn new(cool_looking:u32, musical:u32, aerodynamic:u32, shiny:u32) -> Gear {
        Gear {
            cool_looking,
            musical,
            aerodynamic,
            shiny,
        }
    }
}

struct Rule {
    condition: Box<dyn Fn(&Gear) -> bool>,
    next_workflow: String,
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}



fn read_plan_and_gears(path:&str) -> (Vec<Workflow>, Vec<Gear>){
    let mut workflows = Vec::new();
    let mut gears = Vec::new();
    let mut file = File::open(path).expect("File not found");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    let parts:Vec<&str> = contents.split("\n\n").collect();
    let temp_workflows = parts[0];
    let temp_gears = parts[1];

    for line in temp_workflows.lines() {
        let parts: Vec<&str> = line.split('{').collect();
        let name = parts[0].to_string();
        let rules_str: Vec<&str> = parts[1].split('}').collect();
        let mut rules = Vec::new();
        let rule_strs: Vec<&str> = rules_str[0].split(',').collect();

        for rule_str in &rule_strs[0..rule_strs.len()-1] {
            let rule_parts: Vec<&str> = rule_str.split(':').collect();
            let condition_str = rule_parts[0];
            let next_workflow = rule_parts[1].to_string();
            let condition = Box::new(move |part: &Gear| {
                // parse condition_str into a function and apply it to part
                true
            });
            rules.push(Rule { condition, next_workflow });
        }


        // Handle the last rule separately
        let last_rule = rule_strs[rule_strs.len()-1];
        if last_rule.contains(':') {
            // If the last rule is not "A", "R", or another workflow, process it as usual
            let rule_parts: Vec<&str> = last_rule.split(':').collect();
            let condition_str = rule_parts[0].to_owned();
            let next_workflow = rule_parts[1].to_string().to_owned();

            let condition: Box<dyn for<'r> Fn(&'r Gear) -> bool> = {
                let parts: Vec<&str> = condition_str.split(|c| c == '<' || c == '>').collect();
                let property = parts[0].to_owned();
                let value: u32 = parts[1].to_owned().parse().unwrap();
                if condition_str.contains('<') {
                    Box::new(move |part: &Gear| match property.as_str() {
                        "x" => part.cool_looking < value,
                        "m" => part.musical < value,
                        "a" => part.aerodynamic < value,
                        "s" => part.shiny < value,
                        _ => panic!("Unknown property: {}", property),
                    })
                } else if condition_str.contains('>') {
                    Box::new(move |part: &Gear| match property.as_str() {
                        "x" => part.cool_looking > value,
                        "m" => part.musical > value,
                        "a" => part.aerodynamic > value,
                        "s" => part.shiny > value,
                        _ => panic!("Unknown property: {}", property),
                    })
                } else {
                    panic!("Unknown comparison operator in condition: {}", condition_str);
                }
            };

            rules.push(Rule { condition, next_workflow });
            
        } else {
            // If the last rule is "A", "R", or another workflow, create a rule that always returns true
            let condition: Box<dyn for<'r> Fn(&'r Gear) -> bool> = Box::new(|_| true);
            let next_workflow = match last_rule {
                "A" => "Accept".to_string(),
                "R" => "Reject".to_string(),
                _ => last_rule.to_string(),
            };
            rules.push(Rule { condition, next_workflow });
        }

        workflows.push(Workflow { name, rules });

    }

    for line in temp_gears.lines() {
        let line = line.replace("{x=", "").replace("m=", "").replace("a=", "").replace("s=", "").replace("}", "");
        let parts: Vec<&str> = line.split(',').collect();
        let cool_looking = parts[0].parse::<u32>().unwrap();
        let musical = parts[1].parse::<u32>().unwrap();
        let aerodynamic = parts[2].parse::<u32>().unwrap();
        let shiny = parts[3].parse::<u32>().unwrap();
        gears.push(Gear::new(cool_looking, musical, aerodynamic, shiny));
        
    }

    (workflows, gears)
}



fn main() {
    let (workflows, gears) = read_plan_and_gears("src/test.txt");
    for workflow in workflows {
        println!("{} - {}", workflow.name, workflow.rules.len());
        for rule in workflow.rules {
            println!("{}",  rule.next_workflow);
        }
    }
    for gear in gears {
        println!("{} {} {} {}", gear.cool_looking, gear.musical, gear.aerodynamic, gear.shiny);
    }
    
}
