use std::collections::HashMap;

#[derive(Debug,Clone,PartialEq, Eq)]
pub struct GlobalScope{
 pub variables: HashMap<String,String>

}

#[derive(Debug,Clone,PartialEq, Eq)]
pub struct Variable{
    pub name :String, 
    pub value: String
}
impl Variable{
    pub fn new(name:String,value:String)->Self{
        Self{name,value}
    }
}
impl GlobalScope{
    pub fn new()->Self{
        Self { variables: HashMap::new()}

    }
    pub fn add_global_variable(&mut self,variable_name:String,variable_value:String)->String{
         let existing_variable = self.variables.get(&variable_name);
         if existing_variable.is_some() {
              return String::from("Variable already exist use another variable name");
         }else{

             self.variables.insert(variable_name, variable_value);
              return String::from("Variable saved");

         }

    }
    pub fn get_global_variable(&mut self,variable_name:String)->Option<Variable>{
         let existing_variable = self.variables.get(&variable_name);
         let variable =  self.variables.get_key_value(&variable_name);
         if existing_variable.is_some() {
             return Some(Variable::new(variable.unwrap().0.clone(), variable.unwrap().1.clone()));
         }else{

        return  None;

         }
    }
    pub fn get_all_global_variables(&self)->Option<HashMap<String,String>>{
        if self.variables.is_empty(){
            return None;
        }else{
            return Some(self.variables.clone());
            
        }
    }
}