#[repr(C)]
#[allow(non_camel_case_types)]
struct DDS_DomainParticipantFactory {
    _private: [u8; 0],
}
#[repr(C)]
#[allow(non_camel_case_types)]
struct DDS_DomainParticipantListener {
    _private: [u8; 0],
}
#[repr(C)]
#[allow(non_camel_case_types)]
struct DDS_DomainParticipant {
    _private: [u8; 0],
}

const DDS_STATUS_MASK_NONE: u32 = 0;
const DDS_RETCODE_OK: i32 = 0;

extern "C" {
    fn DDS_DomainParticipantFactory_get_instance() -> *mut DDS_DomainParticipantFactory;
    fn DDS_DomainParticipantFactory_create_participant_with_profile(
        factory: *mut DDS_DomainParticipantFactory,
        domain_id: i32,
        library_name: *const i8,
        profile_name: *const i8,
        participant_listener: *const DDS_DomainParticipantListener,
        status_mask: u32,
    ) -> *mut DDS_DomainParticipant;
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
    let domain_id: i32 = 0;

    let factory = unsafe { DDS_DomainParticipantFactory_get_instance() };

    dbg!("factory address: {:?}", factory);
    if factory == std::ptr::null_mut() {
        panic!("Failed to get DomainParticipantFactory instance");
    }

    let library_name = std::ffi::CString::new("MyLibrary").unwrap();
    let profile_name = std::ffi::CString::new("MyProfile").unwrap();

    let participant = unsafe {
        DDS_DomainParticipantFactory_create_participant_with_profile(
            factory,
            domain_id,
            library_name.as_ptr(),
            profile_name.as_ptr(),
            std::ptr::null(),
            DDS_STATUS_MASK_NONE,
        )
    };

    dbg!("participant address: {:?}", participant);
    if participant == std::ptr::null_mut() {
        panic!("Failed to create DomainParticipant");
    }

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
