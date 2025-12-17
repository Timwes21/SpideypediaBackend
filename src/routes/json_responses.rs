use serde::{Deserialize};
use tracing_subscriber::fmt::format;
use std::collections::HashMap;
use mongodb::{
    bson::{Document, doc}, 
    results::UpdateResult,
    error::Error
};

use crate::routes::route_errors::RouteError;




#[derive(serde::Deserialize, Debug)]
pub struct CharacterLoad{
    pub character: String,
    pub token: String,
}

impl CharacterLoad{
    pub fn get_filter(&self)->Document{
        doc! {"tokens": &self.token}

    }

}



#[derive(Deserialize, Debug)]
pub struct CharacterData {
    pub character: String, 
    #[serde(rename="type")]
    pub title_type: String, 
    #[serde(rename="titleName")]
    pub title_name: String,
    pub vol: Option<String>,
    #[serde(rename="issueNumber")]
    pub issue_number: Option<String>
}

impl CharacterData {
    pub fn get_path(&self)-> String{
        let path = match &self.vol {
            Some(vol) => {
                if let Some(issue_number) = &self.issue_number{
                    format!("characters.{}.{}.{}.{}.{}", 
                                                    self.character,
                                                    self.title_type,
                                                    self.title_name,
                                                    vol,
                                                    issue_number)
                }
                else {format!("characters.{}.{}.{}.vol {}", 
                                                    self.character,
                                                    self.title_type,
                                                    self.title_name,
                                                    vol)}
                
            },
            _ => {
                format!("characters.{}.{}.{}",
                                        self.character,
                                        self.title_type,
                                        self.title_name)

            }
        };

        path
    }


    pub fn get_image_path_issue(&self, username: &String)-> Result<String, RouteError>{
        let image_name = format!("{}/{}/{}/{}/{}/{}", username,
                                    self.character, 
                                    self.title_type,
                                    self.title_name,
                                    self.vol.as_ref().ok_or(RouteError::OptionError)?,
                                    self.issue_number.as_ref().ok_or(RouteError::OptionError)?);
        let image_name = image_name.replace(" ", "-");
        Ok(image_name)
    }


    pub fn get_image_vol_prefix(&self, username: &String) -> Result<String, RouteError>{
        Ok(format!("{}/{}/{}/{}/{}", username, 
                        self.character, 
                        self.title_type,
                        self.title_name,
                        self.vol.as_ref().ok_or(RouteError::OptionError)?))
    }

    

}


#[derive(Deserialize, Debug)]
pub struct AddToCharacterData {
    #[serde(rename="characterData")]
    pub character_data: CharacterData,
    pub token: String,
}

impl AddToCharacterData {
    pub fn get_filter(&self)->Document{
        doc! {"tokens": &self.token}

    }

    pub fn get_update(&self)-> Document{
        let path = self.character_data.get_path();
        let update = doc! {"$set": {path: {}}};
        update
        // const finalResult = await collection.updateOne({tokens : token},{$set: {[key]: {}}});
    }

    pub fn get_remove(&self)->Document{
        let character_data = &self.character_data;
        let path =  {format!("characters.{}.{}.{}.{}", 
                                                    character_data.character,
                                                    character_data.title_type,
                                                    character_data.title_name,
                                                    character_data.vol.as_ref().unwrap())};
        let update = doc! {"$unset": {path: ""}};
        update
    }
}


#[derive(Deserialize, Debug)]
pub struct DeleteIssueData{
    pub token: String,
    #[serde(rename="characterData")]
    pub character_data: CharacterData,
    pub image_name: Option<String>
}

impl DeleteIssueData{
    pub fn get_update(&self)-> Document{
        let path = self.character_data.get_path();
        doc! {"$unset": {path: ""}}
    }
}

impl DeleteIssueData{
    pub fn get_filter(&self)->Document{
        doc! {"tokens": &self.token}

    }

}
