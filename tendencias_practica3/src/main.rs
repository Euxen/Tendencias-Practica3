struct Range{
    base_range: Vec<i32>,
    min_value: i32,
    max_value:i32
}

fn build_range(range_numbers: &str) -> Range{

    let characters:Vec<&str> = range_numbers.split(',').collect();

    let mut lower_limit:Vec<char> = characters[0].to_string().chars().collect::<Vec<char>>();
    let mut upper_limit:Vec<char> = characters[characters.len() - 1].to_string().chars().collect::<Vec<char>>();
    let mut include_lower: bool = false;
    let mut include_upper: bool = false;

    if lower_limit[0] == '['{
        lower_limit.remove(0);
        include_lower = true;  
    }else if lower_limit[0] == '('{
        lower_limit.remove(0);
    }

    if upper_limit[upper_limit.len() - 1] == ']'{
        upper_limit.remove(upper_limit.len()-1);
        include_upper = true;
    }else if upper_limit[upper_limit.len()- 1] == ')'{
        upper_limit.remove(upper_limit.len() -1);
    }

    let beginning_number: i32 = find_if_reverse(lower_limit);
    let ending_number: i32 = find_if_reverse(upper_limit);

    let mut base_range_argument: Vec<i32> = Vec::new();

    if include_lower == true{
        if include_upper == true{
            for i in beginning_number..ending_number+1{
                base_range_argument.push(i);
            }
        }
        else if include_upper == false{
            for i in beginning_number..ending_number{
                base_range_argument.push(i);
            }
        }
    }
    else if include_lower == false{
        if include_upper == true{
            for i in beginning_number+1..ending_number+1{
                base_range_argument.push(i);
            }
        }
        else if include_upper == false{
            for i in beginning_number+1..ending_number{
                base_range_argument.push(i);
            }
        }
    }

    Range{
        base_range: base_range_argument,
        max_value: ending_number,
        min_value: beginning_number
    }

}

fn find_if_reverse(mut num:Vec<char>) -> i32{
    let mut result:i32 = 0;

    if num[0] == '-'{
        num.remove(0);
        let placeholder:String = num.iter().collect();
        let temp: i32 = placeholder.parse().unwrap_or_default(); 
        result = temp - (temp * 2);
    }else if num[0] == ' '{
        num.remove(0);
        let placeholder: String = num.iter().collect();
        result = placeholder.parse().unwrap_or_default();
    }

    result
}

impl Range{

    fn contains(&self, numbers:&str) -> bool{
        let placeholder = build_range(numbers);

        if placeholder.min_value >= self.min_value && placeholder.max_value <= self.max_value{
            return true;
        }
        else{
            return false;
        }
    }

    fn does_not_contain(&self, numbers:&str) ->bool{
        let contain:bool = self.contains(numbers);

        if contain == true{
            return false;
        }
        else{
            return true;
        }
    }

    fn get_all_points(&self){
        print!("{:?}", self.base_range);
    }

    fn contains_range(&self, input_range: &Range) -> bool{
        if input_range.min_value >= self.min_value && input_range.max_value <= self.max_value{
            return true;
        }
        else{
            return false;
        }
    }

    fn does_not_contain_range(&self, input_range: &Range) -> bool{
        let contain:bool = self.contains_range(input_range);

        if contain == true{
            return false;
        }
        else{
            return true;
        }
    }

    fn end_points(&self){
        println!("EndPoints: [{}, {}]", self.min_value, self.max_value);
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
        let equals:bool = self.equals(input_range);

        if equals == true{
            return false;
        }
        else{
            return true;
        }
    }
}

fn main(){

    let range1: &str = "[-8, 0]";
    let range2: &str = "(-7,-4)";
    let range3: &str = "[ 2, 10]";
    let range4: &str = "( 4, 8)";

    //Constructores
    let ejemplo1 = build_range(range1);
    let ejemplo2 = build_range(range2);
    let ejemplo3 = build_range(range3);
    let ejemplo4 = build_range(range4);
    println!("\n"); //newline

    //Contains
    println!("Ejemplo1 contains [-4,-2] = {}", ejemplo1.contains("[-4,-2]"));
    println!("Ejemplo2 contains ( 4, 10) = {}", ejemplo2.contains("( 4, 10)"));
    println!("Ejemplo3 contains ( 2, 10) = {}", ejemplo3.contains("( 2, 10)"));
    println!("Ejemplo4 contains [-2, 0] = {}", ejemplo4.contains("[-2, 0]"));
    println!("\n"); //newline

    //Does not contain
    println!("Ejemplo1 does not contains [-4,-2] = {}", ejemplo1.does_not_contain("[-4,-2]"));
    println!("Ejemplo2 does not contains ( 4, 10) = {}", ejemplo2.does_not_contain("( 4, 10)"));
    println!("Ejemplo3 does not contains ( 2, 10) = {}", ejemplo3.does_not_contain("( 2, 10)"));
    println!("Ejemplo4 does not contains [-2, 0] = {}", ejemplo4.does_not_contain("[-2, 0]"));
    println!("\n"); //newline

    //Get all points
    println!("\n Ejemplo1:");
    ejemplo1.get_all_points();
    println!("\n Ejemplo2:");
    ejemplo2.get_all_points();
    println!("\n Ejemplo3:");
    ejemplo3.get_all_points();
    println!("\n Ejemplo4:");
    ejemplo4.get_all_points();
    println!("\n"); //newline

    //Contains Range
    println!("Ejemplo1 contains Ejemplo 2 = {}", ejemplo1.contains_range(&ejemplo2));
    println!("Ejemplo1 contains Ejemplo 4 = {}", ejemplo1.contains_range(&ejemplo4));
    println!("Ejemplo3 contains Ejemplo 2 = {}", ejemplo3.contains_range(&ejemplo2));
    println!("Ejemplo3 contains Ejemplo 4 = {}", ejemplo3.contains_range(&ejemplo4));
    println!("\n"); //newline
    
    //Does not contain Range
    println!("Ejemplo1 does not contains Ejemplo 2 = {}", ejemplo1.does_not_contain_range(&ejemplo2));
    println!("Ejemplo1 does not contains Ejemplo 4 = {}", ejemplo1.does_not_contain_range(&ejemplo4));
    println!("Ejemplo3 does not contains Ejemplo 2 = {}", ejemplo3.does_not_contain_range(&ejemplo2));
    println!("Ejemplo3 does not contains Ejemplo 4 = {}", ejemplo3.does_not_contain_range(&ejemplo4));
    println!("\n"); //newline

    //Endpoints
    ejemplo1.end_points();
    ejemplo2.end_points();
    ejemplo3.end_points();
    ejemplo4.end_points();
    println!("\n"); //newline

    //Overlaps
    println!("Ejemplo1 overlaps with Ejemplo 2 = {}", ejemplo1.overlaps_range(&ejemplo2));
    println!("Ejemplo1 overlaps with Ejemplo 3 = {}", ejemplo1.overlaps_range(&ejemplo3));
    println!("Ejemplo4 overlaps with Ejemplo 2 = {}", ejemplo4.overlaps_range(&ejemplo2));
    println!("Ejemplo4 overlaps with Ejemplo 3 = {}", ejemplo4.overlaps_range(&ejemplo3));
    println!("\n"); //newline

    //Equals
    println!("Ejemplo1 equals to Ejemplo 1 = {}", ejemplo1.equals(&ejemplo1));
    println!("Ejemplo1 equals to Ejemplo 2 = {}", ejemplo1.equals(&ejemplo2));
    println!("Ejemplo3 equals to Ejemplo 3 = {}", ejemplo3.equals(&ejemplo3));
    println!("Ejemplo3 equals to Ejemplo 4 = {}", ejemplo3.equals(&ejemplo4));
    println!("\n"); //newline

    //Does not Equals
    println!("Ejemplo1 does not equals to Ejemplo 1 = {}", ejemplo1.not_equals(&ejemplo1));
    println!("Ejemplo1 does not equals to Ejemplo 2 = {}", ejemplo1.not_equals(&ejemplo2));
    println!("Ejemplo3 does not equals to Ejemplo 3 = {}", ejemplo3.not_equals(&ejemplo3));
    println!("Ejemplo3 does not equals to Ejemplo 4 = {}", ejemplo3.not_equals(&ejemplo4));
    println!("\n"); //newline
}