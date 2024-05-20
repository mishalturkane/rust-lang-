fn main() {
        let my_trait_object : Box< dyn MakeNoise> = Box:: new (Bird{
            name: "Tweety".to_string(),
            color: "yellow".to_string(),
        });

    my_trait_object.talk();
}
trait MakeNoise{
    fn talk(&self);
}
struct  Bird{
    name: String,
    color: String,
}

impl MakeNoise for Bird {
    fn talk(&self) {
       println!("bird name is {} color is {}",self.name,self.color);
    }
}

fn invite_to_animal_talk(speeker: Box<dyn  MakeNoise>){
    println!("ladies and gentleman , please welcome to ouw next speaker is ");
    speeker.talk();
}