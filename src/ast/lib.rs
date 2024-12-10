use std::{marker::PhantomData, process::Output};


#[macro_export]
macro_rules! id_gen {
    ($name:ident) => {

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, Ord, PartialOrd)]
         pub struct $name(usize);

      impl crate::ast::lib::Id for $name{
        fn new(id:usize)->Self{
            Self(id)
        }

        fn to_usize(&self)->usize{
            self.0
        }
      }
        
    
    };
   
}

pub trait Id: Copy + Clone + Sized{
    fn new(id:usize)->Self;

    fn to_usize(&self)->usize;


}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct IdVec<IdType,T> where IdType:Id{
    pub data:Vec<T>,
    _marker:PhantomData<IdType>
}

impl<IdType:Id,T> IdVec<IdType,T>{
    pub fn new()->Self{
        Self { data: vec![], _marker: PhantomData }
    }

    pub fn get(&self,id:IdType)->&T{
      return &self.data[id.to_usize()];
    }
    pub fn get_mut(&mut self,id:IdType)->&mut T{
      return &mut self.data[id.to_usize()];
    }


    pub fn push(&mut self,item:T)->IdType{

        let index_item_stored = self.data.len();
        self.data.push(item);
        let index_as_idtype = IdType::new(index_item_stored);
        return index_as_idtype;
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
    pub fn is_vec_empty(&self)->bool{
        self.data.is_empty()
    }
}