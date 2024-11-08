#[repr(C)]
#[allow(non_camel_case_types)]
struct DDS_DomainParticipantFactory {
    _private: [u8; 0],
}
#[repr(C)]
#[allow(non_camel_case_types)]
struct DDS_DomainParticipant {
    _private: [u8; 0],
}
#[repr(C)]
#[allow(non_camel_case_types)]
struct DDS_DataWriter {
    _private: [u8; 0],
}
#[repr(C)]
#[allow(non_camel_case_types)]
struct HelloWorld {
    msg: [i8; 256],
}

const DDS_RETCODE_OK: i32 = 0;

extern "C" {
    fn DDS_DomainParticipantFactory_get_instance() -> *mut DDS_DomainParticipantFactory;
    fn HelloWorldTypeSupport_register_type(
        participant: *mut DDS_DomainParticipant,
        type_name: *const i8,
    ) -> i32;
    fn DDS_DomainParticipantFactory_register_type_support(
        DDS_DomainParticipantFactory: *mut DDS_DomainParticipantFactory,
        DDS_DomainParticipantFactory_RegisterTypeFunction: unsafe extern "C" fn(
            *mut DDS_DomainParticipant,
            *const i8,
        ) -> i32,
        type_name: *const i8,
    ) -> i32;
    fn DDS_DomainParticipantFactory_create_participant_from_config(
        factory: *mut DDS_DomainParticipantFactory,
        configuration_name: *const i8,
    ) -> *mut DDS_DomainParticipant;
    fn DDS_DomainParticipant_lookup_datawriter_by_name(
        participant: *mut DDS_DomainParticipant,
        datawriter_full_name: *const i8,
    ) -> *mut DDS_DataWriter;
    fn DDS_DomainParticipant_delete_contained_entities(
        participant: *mut DDS_DomainParticipant,
    ) -> i32;
    fn DDS_DomainParticipantFactory_delete_participant(
        factory: *mut DDS_DomainParticipantFactory,
        participant: *mut DDS_DomainParticipant,
    ) -> i32;
    fn DDS_DomainParticipantFactory_finalize_instance() -> i32;
}

fn main() {
    let factory = unsafe { DDS_DomainParticipantFactory_get_instance() };

    dbg!("factory address: {:?}", factory);
    if factory == std::ptr::null_mut() {
        panic!("Failed to get DomainParticipantFactory instance");
    }

    let type_name = std::ffi::CString::new("HelloWorld").unwrap();
    let retcode = unsafe {
        DDS_DomainParticipantFactory_register_type_support(
            factory,
            HelloWorldTypeSupport_register_type,
            type_name.as_ptr(),
        )
    };
    if retcode != DDS_RETCODE_OK {
        panic!("Failed to register type: {retcode}");
    }

    let participant_name = std::ffi::CString::new("MyParticipantLibrary::MyParticipant").unwrap();
    let participant = unsafe {
        DDS_DomainParticipantFactory_create_participant_from_config(
            factory,
            participant_name.as_ptr(),
        )
    };

    dbg!("participant address: {:?}", participant);
    if participant == std::ptr::null_mut() {
        panic!("Failed to create DomainParticipant");
    }

    let datawriter_name = std::ffi::CString::new("MyPublisher::MyWriter").unwrap();
    let datawriter = unsafe {
        DDS_DomainParticipant_lookup_datawriter_by_name(participant, datawriter_name.as_ptr())
    };

    dbg!("datawriter address: {:?}", datawriter);
    if datawriter == std::ptr::null_mut() {
        panic!("Failed to create DataWriter");
    }

    // Teardown
    let retcode = unsafe { DDS_DomainParticipant_delete_contained_entities(participant) };
    if retcode != DDS_RETCODE_OK {
        panic!("Failed to delete contained entities: {retcode}");
    }

    let retcode = unsafe { DDS_DomainParticipantFactory_delete_participant(factory, participant) };
    if retcode != DDS_RETCODE_OK {
        panic!("Failed to delete participant: {retcode}");
    }

    let retcode = unsafe { DDS_DomainParticipantFactory_finalize_instance() };
    if retcode != DDS_RETCODE_OK {
        panic!("FFailed to finalize participant factory: {retcode}");
    }
}
