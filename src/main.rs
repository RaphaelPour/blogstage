use std::fs;

fn main(){
    /* use first argument as server directory where all the files are placed
     * fail with proper error message otherwise */
    let dir = match std::env::args().nth(1){
        Some(dir) => dir,
        None => {
            println!("usage: blogstage <dir>");
            return
        }
    };

    /* create new web_server before adding all the routes */
    let server = web_server::new();
    
    /* iterate through the given directory and add a route for every path
     * where the route equals the path
     * this should should be enough for a simple web server
     */
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();

        server.get(entry.file_name().to_str().unwrap(), Box::new(|_,_| {
            entry.path().to_str().unwrap().into()
        }));
    }

    /* start server blocking */
    server.launch(8080);
}
