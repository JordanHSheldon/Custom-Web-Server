// rustc main.rs <-- compile
// ./main        <-- Execute

// struct representing backend server
struct Backend {
    active: bool
}

// type representing all the backend servers available
struct Backends {
    count: u16,
    backends: Vec<Backend>
}

impl Backends {
    fn add_server(&mut self,server: Backend) {
        self.backends.push(server);
        self.count+=1;
    }

    fn remove_server(&mut self){
        self.backends.pop();
        self.count-=1;
    }
}

fn main() {
    let new_backend = Backend{active:true};
    let mut backend_servers = Backends{
        backends: Vec::<Backend>::new(),
        count: 0
    };
    
    backend_servers.add_server(new_backend);

   println!("{} server('s)", backend_servers.count);

   backend_servers.remove_server();

   println!("{} server('s)", backend_servers.count);
}
