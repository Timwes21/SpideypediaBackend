async fn login(){

}


async fn logout(){

}


async fn create_user(request: Request){
    let body = request.body();
    println!("In creating user route user");
        
    // try{
    //     const token = await createUser(data, collection);
    //     res.status(200).json({token: token});
    // }
    // catch(err){
    //     res.status(500).send({err: err});
    //     console.log(err);
    // }
}

