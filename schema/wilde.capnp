@0xd6d3c8c846976e4e;

struct Message(T) {
    id @0 :Text;
    data @1 :T;
}

interface Producer(T) {
    publish @0 (data :T) -> ();
}

interface Consumer(T) {
    consume @0 () -> (message :Message(T));
    confirm @1 (message :Message(T)) -> ();
}
