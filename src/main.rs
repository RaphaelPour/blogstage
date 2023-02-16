use std::{fs, io};

fn main() -> io::Result<()>{
    /* use first argument as server directory where all the files are placed
     * fail with proper error message otherwise */
    let dir = std::env::args().nth(1).expect("serve directory missing");

    /* create new web_server before adding all the routes */
    let server = web_server::new();
    
    /* iterate through the given directory and add a route for every path
     * where the route equals the path
     * this should should be enough for a simple web server
     */
    fs::read_dir(dir)?
        .map(|res| {
            res.map(|e| {
                server.get(e.path(), Box::new(|_,_| {
                    std::path::Path::new(&e.path()).into()
                }))
            })
        });

    /* start server blocking */
    server.launch(8080);

    Ok(())
}
