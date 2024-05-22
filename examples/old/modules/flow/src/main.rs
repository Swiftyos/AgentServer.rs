extern crate flatbuffers;

mod fb_generated;

use flatbuffers::FlatBufferBuilder;
use fb_generated::users::{User, UserArgs, finish_user_buffer, root_as_user};


pub fn make_user(bldr: &mut FlatBufferBuilder, dest: &mut Vec<u8>, name: &str, id: u64) {
    // Reset the `bytes` Vec to a clean state.
    dest.clear();

    // Reset the `FlatBufferBuilder` to a clean state.
    bldr.reset();

    // Create a temporary `UserArgs` object to build a `User` object.
    // (Note how we call `bldr.create_string` to create the UTF-8 string
    // ergonomically.)
    let args = UserArgs{
        name: Some(bldr.create_string(name)),
        id: id,
    };

    // Call the `User::create` function with the `FlatBufferBuilder` and our
    // UserArgs object, to serialize the data to the FlatBuffer. The returned
    // value is an offset used to track the location of this serializaed data.
    let user_offset = User::create(bldr, &args);

    // Finish the write operation by calling the generated function
    // `finish_user_buffer` with the `user_offset` created by `User::create`.
    finish_user_buffer(bldr, user_offset);

    // Copy the serialized FlatBuffers data to our own byte buffer.
    let finished_data = bldr.finished_data();
    dest.extend_from_slice(finished_data);
}
pub fn read_user(buf: &[u8]) -> (&str, u64) {
    let u = root_as_user(buf);
    let name = u.name().unwrap();
    let id = u.id();
    (name, id)
}

fn main() {
    let mut bldr = FlatBufferBuilder::new();
    let mut bytes: Vec<u8> = Vec::new();

    // Write the provided `name` and `id` into the `bytes` Vec using the
    // FlatBufferBuilder `bldr`:
    make_user(&mut bldr, &mut bytes, "Arthur Dent", 42);

    // Now, `bytes` contains the serialized representation of our User object.

    // To read the serialized data, call our `read_user` function to decode
    // the `user` and `id`:
    let (name, id) = read_user(&bytes[..]);

    // Show the decoded information:
    println!("{} has id {}. The encoded data is {} bytes long.", name, id, bytes.len());
}
