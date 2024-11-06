#[repr(C)]
#[allow(non_camel_case_types)]
struct DDS_DomainParticipantFactory {_private: [u8; 0]}

extern {
    fn DDS_DomainParticipantFactory_get_instance() -> *mut DDS_DomainParticipantFactory;
}

fn main() {
    let x = unsafe { DDS_DomainParticipantFactory_get_instance() };
    println!("factory address: {:?}", x);
}