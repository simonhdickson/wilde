@0xd6d3c8c846976e4e;

struct Message(T) {
    id @0 :Text;
    data @1 :T;
}

interface PublisherFunction(T) {
    call @0 (message :Message(T)) -> ();
}

interface Publisher(T) {
    create @0 (topic :Text) -> (publishFunc :PublisherFunction(T));
}

interface ConsumerFunction(T) {
    call @0 (message :Message(T)) -> (processed :Bool);
}

interface Consumer(T) {
    subscribe @0 (topic :Text, consumerGroup :Text, consumer :Text, consumeFunc :ConsumerFunction(T)) -> ();
}
