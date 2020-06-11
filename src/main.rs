pub struct Range{
    total_range: Vec<i32>,
    lower_limit: i32,
    upper_limit: i32
}

impl Range{

    fn new(input: &str) -> Self{

        let characters: Vec<&str> = input.split(',').collect();//[-8 || 0]

        let mut lower_limit: Vec<char> = characters[0].to_string().chars().collect::<Vec<char>>(); //[-8
        let mut upper_limit: Vec<char> = characters[characters.len()-1].to_string().chars().collect::<Vec<char>>();// 0]

        let mut include_lower = false;
        let mut include_upper = false;

        if lower_limit[0] == '['{
            lower_limit.remove(0);
            include_lower = true;
        }else if lower_limit[0] == '('{
            lower_limit.remove(0);
        }
        
        

        if upper_limit[upper_limit.len()-1] == ']'{
            upper_limit.remove(upper_limit.len()-1);
            include_upper = true;
        }else if upper_limit[upper_limit.len()-1] == ')'{
            upper_limit.remove(upper_limit.len()-1);
        }

        // -8 || 0

        let mut beginning: i32 = find_limit_number(&mut lower_limit); 
        let mut ending: i32 = find_limit_number(&mut upper_limit); 

        let mut object_range: Vec<i32> = Vec::new();

        if include_lower == true{
            if include_upper == true{
                for i in beginning..ending + 1{
                    object_range.push(i);
                }
            }
            else if include_upper == false{
                for i in beginning..ending{
                    object_range.push(i);
                }
            }
        }
        else if include_lower == false{ //(3,7] 4,5,6,7
            if include_upper == true{
                for i in beginning + 1..ending + 1{
                    object_range.push(i);
                }
            }
            else if include_upper == false{
                for i in beginning + 1 .. ending{
                    object_range.push(i);
                }
            }
        }

        //Validation

        if object_range.len() > 0{
            if object_range[0] != beginning{
                beginning = beginning + 1;
            }
            if object_range[object_range.len() - 1] != ending{
                ending = ending - 1;
            }
        }

        


        Range{
            total_range: object_range,
            lower_limit: beginning,
            upper_limit: ending
        }
    
    }

    fn contains(&self, input: &str) -> bool{
        let input_object = Range::new(input);

        //[2,6) contains {2,4}
        // 2,3,4,5 contiene 2,3,4
        //[2,6) doesn’t contain {-1,1,6,10}
        // 2,3,4,5 no contiene -1, 1, 6, 10
        // limite inferior del input mayor o igual al self.lowerlimit y limite superior del input menor o igual al self.upperlimit
        
        if input_object.lower_limit >= self.lower_limit && input_object.upper_limit <= self.upper_limit{
            return true;
        }
        else{
            return false;
        }

    }
    
    fn doesnt_contains(&self, input: &str) -> bool{
        let contain_bool: bool = self.contains(input);

        if contain_bool == true{
            return false;
        }
        else{
            return true;
        }
    }

    fn get_all_points(&self) -> &Vec<i32>{
        &self.total_range
    }

    fn contains_range(&self, input: Range) -> bool{
        if input.lower_limit >= self.lower_limit && input.upper_limit <= self.upper_limit{
            return true;
        }
        else{
            return false;
        }
    }

    fn does_not_contains_range(&self, input: Range) -> bool{
        let contain_bool: bool = self.contains_range(input);

        if contain_bool == true{
            return false;
        }
        else{
            return true;
        }
    }

    fn get_endpoints(&self) -> Vec<i32>{
        let mut result: Vec<i32> = Vec::new();
        result.push(self.lower_limit);
        result.push(self.upper_limit);
        result
    }
    
    fn overlaps_range(&self, input: Range) -> bool{
        
        for i in self.total_range.iter() {
            for a in input.total_range.iter(){
                if a == i{
                    return true;
                    
                }
            }
        }

        return false;

    }

    fn equals_range(&self, input: Range) -> bool{
        if self.total_range == input.total_range{
            return true;
        }
        else{
            return false;
        }
    }

    fn not_equals_range(&self, input:Range) -> bool{
            let equals_bool: bool = self.equals_range(input);

            if equals_bool == true{
                return false;
            }
            else{
                return true;
            }
    }
}

pub fn find_limit_number(input_array: &mut Vec<char>) -> i32{
    let mut result: i32 = 0;

    if input_array[0] == '-'{
        input_array.remove(0);
        let placeholder: String = input_array.iter().collect();
        let tempint: i32 = placeholder.parse().unwrap_or_default();
        result = tempint - (tempint * 2);
    }
    else if input_array[0] == ' '{
        input_array.remove(0);
        let placeholder: String = input_array.iter().collect();
        result = placeholder.parse().unwrap_or_default();
    }
    else{
        let placeholder: String = input_array.iter().collect();
        result = placeholder.parse().unwrap_or_default();
    }

    result
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1_constructor_takes_negative_numbers_and_brackets(){

        let expected_input: &str = "[-8, 0]";
        let mut expected_range: Vec<i32> = Vec::new();

        for i in -8..0 + 1{
            expected_range.push(i);
        }

        let expected_object: Range = Range{total_range : expected_range, upper_limit: 0, lower_limit: -8};
        
        assert_eq!(Range::new(expected_input).total_range, expected_object.total_range);
    }

    #[test]
    fn test2_constructor_takes_negative_numbers_and_parenthesis(){
        let expected_input: &str = "(-7,-4)"; //-6,-5
        let mut expected_range: Vec<i32> = Vec::new();

        // for (int i = 0, i < -4, i ++)
        for i in -6..-4{
            expected_range.push(i);
        }

        let expected_object: Range = Range{total_range : expected_range, upper_limit: -5, lower_limit: -6};
        
        assert_eq!(Range::new(expected_input).total_range, expected_object.total_range);
    }

    #[test]
    fn test3_constructor_takes_postitive_numbers_and_brackets(){
        let expected_input: &str = "[ 2, 10]"; 
        let mut expected_range: Vec<i32> = Vec::new();

        // for (int i = 0, i < -4, i ++)
        for i in 2..10 + 1{
            expected_range.push(i);
        }

        let expected_object: Range = Range{total_range : expected_range, upper_limit: 10, lower_limit: 2};
        
        assert_eq!(Range::new(expected_input).total_range, expected_object.total_range);
    }

    #[test]
    fn test4_constructor_takes_postitive_numbers_and_parenthesis(){
        let expected_input: &str = "( 4, 8)";//5,6,7
        let mut expected_range: Vec<i32> = Vec::new();

        // for (int i = 0, i < -4, i ++)
        for i in 4 + 1..8{
            expected_range.push(i);
        }

        let expected_object: Range = Range{total_range : expected_range, upper_limit: 7, lower_limit: 5};
        
        assert_eq!(Range::new(expected_input).total_range, expected_object.total_range);
    }

    
    #[test]
    fn test5_contains_negative_numbers_with_brackets(){
        let expected_input: &str = "[-4,-2]";
        let ejemplo1: Range = Range::new("[-8, 0]");

        assert_eq!(ejemplo1.contains(expected_input), true);
    } 

    #[test]
    fn test6_contains_negative_numbers_with_parenthesis(){
        let expected_input: &str = "(4, 10)";
        let ejemplo1: Range = Range::new("(-7,-4)");

        assert_eq!(ejemplo1.contains(expected_input), false);
    }
    
    #[test]
    fn test7_contains_positive_numbers_with_parenthesis(){
        let expected_input: &str = "(2, 10)";
        let ejemplo1: Range = Range::new("[2, 10]");

        assert_eq!(ejemplo1.contains(expected_input), true);
    } 
    
    #[test]
    fn test8_contains_positive_numbers_with_brackets(){
        let expected_input: &str = "[-2,0]";
        let ejemplo1: Range = Range::new("(4,8)");//5 & 7

        assert_eq!(ejemplo1.contains(expected_input), false);
    } 
    
    #[test]
    fn test9_does_not_contains_negative_numbers_with_brackets(){
        let expected_input: &str = "[-4,-2]";
        let ejemplo1: Range = Range::new("[-8, 0]");

        assert_eq!(ejemplo1.doesnt_contains(expected_input), false);
    } 

    #[test]
    fn test10_contains_negative_numbers_with_parenthesis(){
        let expected_input: &str = "(4, 10)";
        let ejemplo1: Range = Range::new("(-7,-4)");

        assert_eq!(ejemplo1.doesnt_contains(expected_input), true);
    }
    
    #[test]
    fn test11_contains_positive_numbers_with_parenthesis(){
        let expected_input: &str = "(2, 10)";
        let ejemplo1: Range = Range::new("[2, 10]");

        assert_eq!(ejemplo1.doesnt_contains(expected_input), false);
    } 
    
    #[test]
    fn test12_contains_positive_numbers_with_brackets(){
        let expected_input: &str = "[-2,0]";
        let ejemplo1: Range = Range::new("(4,8)");

        assert_eq!(ejemplo1.doesnt_contains(expected_input), true);
    } 
       
    #[test]
    fn test13_get_all_points_from_range() {
        let mut expected_output: Vec<i32> = Vec::new();

        for i in -8..0 + 1{
            expected_output.push(i);
        }

        let ejemplo1 = Range::new("[-8,0]");

        assert_eq!(ejemplo1.get_all_points(), &expected_output)
    }
    
    #[test]
    fn test14_get_all_points_from_range() {
        let mut expected_output: Vec<i32> = Vec::new();

        for i in -6..-4{ //-6, -5
            expected_output.push(i);
        }

        let ejemplo1 = Range::new("(-7,-4)");

        assert_eq!(ejemplo1.get_all_points(), &expected_output)
    }

    #[test]
    fn test15_get_all_points_from_range() {
        let mut expected_output: Vec<i32> = Vec::new();

        for i in 2..10 + 1{ //[2,10]
            expected_output.push(i);
        }

        let ejemplo1 = Range::new("[ 2, 10]");

        assert_eq!(ejemplo1.get_all_points(), &expected_output)
    }

    #[test]
    fn test16_get_all_points_from_range() {
        let mut expected_output: Vec<i32> = Vec::new();

        for i in 5..7 + 1{ //[5,7]
            expected_output.push(i);
        }

        let ejemplo1 = Range::new("( 4, 8)");

        assert_eq!(ejemplo1.get_all_points(), &expected_output)
    }

    #[test]
    fn test17_contains_another_range(){

        let ejemplo1 = Range::new("[-8, 0]");
        let ejemplo2 = Range::new("(-7,-4)");

        assert_eq!(ejemplo1.contains_range(ejemplo2), true);
    }

    #[test]
    fn test18_contains_another_range(){

        let ejemplo1 = Range::new("[-8, 0]");
        let ejemplo2 = Range::new("( 4, 8)");

        assert_eq!(ejemplo1.contains_range(ejemplo2), false);
    }

    #[test]
    fn test19_contains_another_range(){

        let ejemplo1 = Range::new("[2, 10]");
        let ejemplo2 = Range::new("( 4, 8)");

        assert_eq!(ejemplo1.contains_range(ejemplo2), true);
    }

    #[test]
    fn test20_contains_another_range(){
        let ejemplo1 = Range::new("[2, 10]"); // 2 >> 10
        let ejemplo2 = Range::new("(-7,-4)");// -6, -5

        assert_eq!(ejemplo1.contains_range(ejemplo2), false);
    }

    #[test]
    fn test21_does_not_contain_another_range() {
        let ejemplo1 = Range::new("[-8, 0]");
        let ejemplo2 = Range::new("(-7,-4)");

        assert_eq!(ejemplo1.does_not_contains_range(ejemplo2), false);

    }

    #[test]
    fn test22_does_not_contain_another_range() {
        let ejemplo1 = Range::new("[-8, 0]");
        let ejemplo2 = Range::new("( 4, 8)");

        assert_eq!(ejemplo1.does_not_contains_range(ejemplo2), true);

    }

    
    #[test]
    fn test23_does_not_contain_another_range() {
        let ejemplo1 = Range::new("[ 2, 10]");
        let ejemplo2 = Range::new("( 4, 8)");

        assert_eq!(ejemplo1.does_not_contains_range(ejemplo2), false);

    }
      
    #[test]
    fn test24_does_not_contain_another_range() {
        let ejemplo1 = Range::new("[ 2, 10]");
        let ejemplo2 = Range::new("(-7,-4)");

        assert_eq!(ejemplo1.does_not_contains_range(ejemplo2), true);

    }

    #[test]
    fn test_25_get_all_endpoints(){
        let ejemplo1 = Range::new("[-8, 0]");
        let mut expected_output: Vec<i32> = Vec::new();
        expected_output.push(-8);
        expected_output.push(0);

        assert_eq!(ejemplo1.get_endpoints(), expected_output);
    }

    #[test]
    fn test_26_get_all_endpoints(){
        let ejemplo1 = Range::new("(-7,-4)");
        let mut expected_output: Vec<i32> = Vec::new();
        expected_output.push(-6);
        expected_output.push(-5);

        assert_eq!(ejemplo1.get_endpoints(), expected_output);
    }

    #[test]
    fn test_27_get_all_endpoints(){
        let ejemplo1 = Range::new("[ 2, 10]");
        let mut expected_output: Vec<i32> = Vec::new();
        expected_output.push(2);
        expected_output.push(10);

        assert_eq!(ejemplo1.get_endpoints(), expected_output);
    }

    #[test]
    fn test_28_get_all_endpoints(){
        let ejemplo1 = Range::new("( 4, 8)");
        let mut expected_output: Vec<i32> = Vec::new();
        expected_output.push(5);
        expected_output.push(7);

        assert_eq!(ejemplo1.get_endpoints(), expected_output);
    }

    #[test]
    fn test_29_range_overlaps_range(){
        let ejemplo1 = Range::new("[-8,0]");
        let ejemplo2 = Range::new("(-7,-4)");

        assert_eq!(ejemplo1.overlaps_range(ejemplo2), true);
    }

    #[test]
    fn test_30_range_overlaps_range(){
        let ejemplo1 = Range::new("[-8,0]");
        let ejemplo2 = Range::new("[2, 10]");

        assert_eq!(ejemplo1.overlaps_range(ejemplo2), false);
    }

    #[test]
    fn test_31_range_overlaps_range(){
        let ejemplo1 = Range::new("(4,8)");
        let ejemplo2 = Range::new("(-7,-4)");

        assert_eq!(ejemplo1.overlaps_range(ejemplo2), false);
    }

    #[test]
    fn test_32_range_overlaps_range(){
        let ejemplo1 = Range::new("(4,8)");
        let ejemplo2 = Range::new("[2, 10]");

        assert_eq!(ejemplo1.overlaps_range(ejemplo2), true);
    }

    #[test]
    fn test_33_range_equals_range(){
        let ejemplo1 = Range::new("[-8,0]");
        let ejemplo2 = Range::new("[-8,0]");

        assert_eq!(ejemplo1.equals_range(ejemplo2), true);
    }

    #[test]
    fn test_34_range_equals_range(){
        let ejemplo1 = Range::new("[-8,0]");
        let ejemplo2 = Range::new("(-7,-4)");

        assert_eq!(ejemplo1.equals_range(ejemplo2), false);
    }

    #[test]
    fn test_35_range_equals_range(){
        let ejemplo1 = Range::new("[2,10]");
        let ejemplo2 = Range::new("[2,10]");

        assert_eq!(ejemplo1.equals_range(ejemplo2), true);
    }

    #[test]
    fn test_36_range_equals_range(){
        let ejemplo1 = Range::new("[2,10]");
        let ejemplo2 = Range::new("[4,8]");

        assert_eq!(ejemplo1.equals_range(ejemplo2), false);
    }

    #[test]
    fn test_37_range_does_not_equals_range(){
        let ejemplo1 = Range::new("[-8,0]");
        let ejemplo2 = Range::new("[-8,0]");

        assert_eq!(ejemplo1.not_equals_range(ejemplo2), false);
    }

    #[test]
    fn test_38_range_does_not_equals_range(){
        let ejemplo1 = Range::new("[-8,0]");
        let ejemplo2 = Range::new("[-7,-4]");

        assert_eq!(ejemplo1.not_equals_range(ejemplo2), true);
    }

    #[test]
    fn test_39_range_does_not_equals_range(){
        let ejemplo1 = Range::new("[2,10]");
        let ejemplo2 = Range::new("[2,10]");

        assert_eq!(ejemplo1.not_equals_range(ejemplo2), false);
    }

    #[test]
    fn test_40_range_does_not_equals_range(){
        let ejemplo1 = Range::new("[2,10]");
        let ejemplo2 = Range::new("[4,8]");

        assert_eq!(ejemplo1.not_equals_range(ejemplo2), true);
    }
}