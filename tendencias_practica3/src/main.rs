struct Range{
    base_range: Vec<i32>,
    min_value: i32,
    max_value: i32

}

impl Range{

    
    fn contains(&self, numbers: &str) -> bool{
        let placeholder = build_range(numbers);

        if placeholder.min_value >= self.min_value && placeholder.max_value <= self.max_value{
            return true;
        }
        else{
            return false;
        }
    }

    
    fn does_not_contains(&self, numbers: &str) -> bool{
        let contain: bool= self.contains(numbers);

        if contain == true{
            return false;
        }
        else{
            return true;
        }


        
    }
    

    fn all_points(&self){
        
        println!("{:?}", self.base_range);
        
    }
    
    fn contains_range(&self, input_range: &Range) -> bool{
        if input_range.min_value >= self.min_value && input_range.max_value <= self.max_value{
            return true;
        }
        else{
            return false;
        }

    }

    fn doesnt_contains_range(&self, input_range: &Range) -> bool{
        let contain:bool = self.contains_range(input_range);
        if contain == true{
            return false;
        }
        else{
            return true;
        }
    }
    
    fn end_points(&self){
        println!("EndPoints: [{} , {}]", self.min_value, self.max_value);
    }
    
    fn overlaps_range(&self, input_range: &Range) -> bool{

        let mut list_number: Vec<&i32> = Vec::new();


        for i in self.base_range.iter(){
            for next in input_range.base_range.iter(){
                if i == next{
                    list_number.push(i);
                }
            }
        }

        if list_number.len() > 0{
            return true;
        }
        else{
            return false;
        }
        
        

        
    }

    
    fn equals(&self, input_range: &Range) -> bool{
        if self.base_range == input_range.base_range{
            return true;
        }
        else{
            return false;
        }
    }
    
    fn not_equals(&self, input_range: &Range) -> bool{
        let check_bool = self.equals(input_range);

        if check_bool == true{
            return false;
        }
        else{
            return true;
        }
    }
    
    
}

fn build_range(range_numbers: &str) -> Range{
    
    let characters: Vec<&str> = range_numbers.split(',').collect();
    let mut lower_limit: Vec<char> = characters[0].to_string().chars().collect::<Vec<char>>();
    let mut upper_limit: Vec<char> = characters[characters.len()-1].to_string().chars().collect::<Vec<char>>();
    let mut include_lower: bool = false;
    let mut include_upper: bool = false;

    if lower_limit[0] == '['{
        lower_limit.remove(0);
        include_lower = true;
    }else if lower_limit[0] == '('{
        lower_limit.remove(0);
    }

    if upper_limit[upper_limit.len() - 1] == ']'{
        upper_limit.remove(upper_limit.len()- 1);
        include_upper = true;
    }else if upper_limit[upper_limit.len()- 1] == ')'{
        upper_limit.remove(upper_limit.len()- 1);
    }

    let  beginning_number: i32 = find_if_reverse(lower_limit);
    let  ending_number: i32 = find_if_reverse(upper_limit);
    
    let mut base_range_argument: Vec<i32> = Vec::new();

    if include_lower == true{
        if include_upper == true{
            for i in beginning_number..ending_number + 1{
                base_range_argument.push(i);
            }
        }
        else if include_upper == false{
            for i in beginning_number..ending_number{
                base_range_argument.push(i)
            }
        }
    }
    else if include_lower == false{
        if include_upper == true{
            for i in beginning_number + 1..ending_number + 1{
                base_range_argument.push(i);
            }
        }
        else if include_upper == false{
            for i in beginning_number + 1..ending_number{
                base_range_argument.push(i)
            }
        }
    }

    
    Range {
        base_range: base_range_argument,
        max_value: ending_number,
        min_value: beginning_number
    }
}

fn find_if_reverse(mut num: Vec<char>) -> i32{
    
    let mut result: i32 = 0;

    if num[0] == '-'{
        num.remove(0);
        let placeholder:String = num.iter().collect();

        let temp: i32 = placeholder.parse().unwrap_or_default();
        result = temp - (temp * 2);
        
    }else if num[0] == ' '{
        num.remove(0);
        let placeholder:String = num.iter().collect();
        result = placeholder.parse().unwrap_or_default()
    }

    result
}

fn main() { 
    
    let range1: &str = "[-8, 0]";
    let range2: &str = "(-7,-4)";
    let range3: &str = "[ 2, 10]";
    let range4: &str = "( 4, 8)";

    //Constructores
    let ejemplo1 = build_range(range1);
    let ejemplo2 = build_range(range2);
    let ejemplo3 = build_range(range3);
    let ejemplo4 = build_range(range4);
    println!("\n");

    //Contains
    println!("Ejemplo1 contains [-4,-2] = {}", ejemplo1.contains("[-4,-2]"));
    println!("Ejemplo2 contains (4,10) = {}", ejemplo2.contains("(4,10)"));
    println!("Ejemplo3 contains (2,10) = {}", ejemplo3.contains("(2,10)"));
    println!("Ejemplo4 contains [-2,0] = {}", ejemplo4.contains("[-2,0]"));
    println!("\n");

    //Does not contain
    println!("Ejemplo1 does_not_contain [-4,-2] = {}", ejemplo1.does_not_contains("[-4,-2]"));
    println!("Ejemplo2 does_not_contain (4,10) = {}", ejemplo2.does_not_contains("(4,10)"));
    println!("Ejemplo3 does_not_contain (2,10) = {}", ejemplo3.does_not_contains("(2,10)"));
    println!("Ejemplo4 does_not_contain [-2,0] = {}", ejemplo4.does_not_contains("[-2,0]"));
    println!("\n");

    //Get all Points
    
    println!("\n Ejemplo 1:");
    ejemplo1.all_points();
    println!("\n Ejemplo 2:");
    ejemplo2.all_points();
    println!("\n Ejemplo 3:");
    ejemplo3.all_points();
    println!("\n Ejemplo 4:");
    ejemplo4.all_points();
    println!("\n");

    //Contains Range
    println!("Ejemplo1 contain Ejemplo2 = {}", ejemplo1.contains_range(&ejemplo2));
    println!("Ejemplo1 contain Ejemplo4 = {}", ejemplo1.contains_range(&ejemplo4));
    println!("Ejemplo3 contain Ejemplo2 = {}", ejemplo1.contains_range(&ejemplo2));
    println!("Ejemplo3 contain Ejemplo4 = {}", ejemplo1.contains_range(&ejemplo4));
    println!("\n");

    //Does not contain Range
    println!("Ejemplo1 doesn't contain Ejemplo2 = {}", ejemplo1.doesnt_contains_range(&ejemplo2));
    println!("Ejemplo1 doesn't contain Ejemplo4 = {}", ejemplo1.doesnt_contains_range(&ejemplo4));
    println!("Ejemplo3 doesn't contain Ejemplo2 = {}", ejemplo1.doesnt_contains_range(&ejemplo2));
    println!("Ejemplo3 doesn't contain Ejemplo4 = {}", ejemplo1.doesnt_contains_range(&ejemplo4));
    println!("\n");

    //Endpoints
    ejemplo1.end_points();
    ejemplo2.end_points();
    ejemplo3.end_points();
    ejemplo4.end_points();
    println!("\n");

    //Overlaps
    println!("Ejemplo1 overlaps with Ejemplo2 = {}", ejemplo1.overlaps_range(&ejemplo2));
    println!("Ejemplo1 overlaps with Ejemplo3 = {}", ejemplo1.overlaps_range(&ejemplo3));
    println!("Ejemplo4 overlaps with Ejemplo2 = {}", ejemplo4.overlaps_range(&ejemplo2));
    println!("Ejemplo4 overlaps with Ejemplo3 = {}", ejemplo4.overlaps_range(&ejemplo3));
    println!("\n");

    //Equals
    println!("Ejemplo1 equals to Ejemplo1 = {}", ejemplo1.equals(&ejemplo1));
    println!("Ejemplo1 equals to Ejemplo2 = {}", ejemplo1.equals(&ejemplo2));
    println!("Ejemplo3 equals to Ejemplo3 = {}", ejemplo3.equals(&ejemplo3));
    println!("Ejemplo3 equals to Ejemplo4 = {}", ejemplo3.equals(&ejemplo4));
    println!("\n");

    //Not Equals
    println!("Ejemplo1 doesn't equals to Ejemplo1 = {}", ejemplo1.not_equals(&ejemplo1));
    println!("Ejemplo1 doesn't equals to Ejemplo2 = {}", ejemplo1.not_equals(&ejemplo2));
    println!("Ejemplo3 doesn't equals to Ejemplo3 = {}", ejemplo3.not_equals(&ejemplo3));
    println!("Ejemplo3 doesn't equals to Ejemplo4 = {}", ejemplo3.not_equals(&ejemplo4));
    println!("\n");
}
