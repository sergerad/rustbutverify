#[derive(Debug)]
struct Data {
    a: String,
}

impl Data {
    fn new(a: String) -> Data {
        Data { a }
    }
}

#[derive(Debug)]
struct Datastore {
    data: Vec<Data>,
}

impl Datastore {
    fn new() -> Datastore {
        Datastore { data: Vec::new() }
    }

    fn add(&mut self, data: Data) {
        self.data.push(data);
    }

    fn get(&self, index: usize) -> &Data {
        &self.data[index]
    }
}

fn main() {
    // Get a mutable reference to the Datastore
    let mut ds = Datastore::new();
    let d = Data::new("hello".to_string());
    ds.add(d);

    // Derive some data from the Datastore. The lifetime of which is
    // implicitly tied to the lifetime of the Datastore
    let dd_a = {
        // We need this shared reference to be out of scope before
        // we use the mutable reference of Datastore again
        let dd = ds.get(0);
        dd.a.clone()
    };
    println!("{:?}", dd_a);

    // We can do this because the shared reference is out of scope
    let ddd = Data::new("world".to_string());
    ds.add(ddd);
}
