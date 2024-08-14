use std::io;

use async_std::{fs::read_to_string, task::block_on};

#[async_std::main]
async fn main() -> Result<(), io::Error> {
    let f = async {
        let s = read_to_string("Cargo.toml").await?;  // 测试中文
        println!("{}", s);
        Ok::<(), io::Error>(())
    };

    block_on(f)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{marker::PhantomPinned, pin::Pin};

    #[test]
    fn test_string() {
        let s = String::from("hello world");
        let ref_s: &String = &s;
        println!("s = {}; ref_s = {}", s, ref_s);

        // let slice_s = Vec::from(s[0..]);
        // println!("slice_s = {:?}", slice_s);
    }

    struct RefSelfData {
        val: String,
        ref_val: *const String,
    }

    impl RefSelfData {
        fn new(data: &str) -> Self {
            Self { val: data.to_string(), ref_val: std::ptr::null() }
        }

        fn init_self_ref(&mut self) {
            self.ref_val = &self.val;
        }
    }

    #[test]
    fn test_pin_unpin() {
        let mut guan = RefSelfData::new("guan");
        guan.init_self_ref();

        {
            let mut zhong = RefSelfData::new("zhong");
            zhong.init_self_ref();

            guan = zhong;
            guan.val = String::from("yang");
        }

        // zhong.val 指向的内容被释放 ; guan.ref_val 成了悬垂指针
        println!(
            "val = {}, val_p = {}, ref_val = {}, ref_val_p = {}",
            &guan.val, &guan.val as *const String as usize,
            unsafe { &*(guan.ref_val) }, guan.ref_val as usize,
        );
    }
    
    struct RefSelfDataPinned {
        val: String,
        ref_val: *const String,
        _marker: PhantomPinned,
    }

    impl RefSelfDataPinned {
        fn new(data: &str) -> Self {
            Self { val: data.to_string(), ref_val: std::ptr::null(), _marker: PhantomPinned }
        }

        fn init_self_ref(self: Pin<&mut Self>) {
            let self_ptr: *const String = &self.val;
            let this = unsafe { self.get_unchecked_mut() };
            this.ref_val = self_ptr;
        }
    }

    #[test]
    fn test_use_pin() {
        let mut guan = RefSelfDataPinned::new("guan");
        let mut guan_pinned = unsafe { Pin::new_unchecked(&mut guan) } ;
        RefSelfDataPinned::init_self_ref(guan_pinned.as_mut());

        let mut zhong = RefSelfDataPinned::new("zhong");
        let mut zhong_pinned = unsafe { Pin::new_unchecked(&mut zhong) } ;
        RefSelfDataPinned::init_self_ref(zhong_pinned.as_mut());

        // guan_pinned.get_mut().val = zhong_pinned.get_mut().val;

        // zhong.val 指向的内容被释放 ; guan.ref_val 成了悬垂指针
        // println!(
        //     "val = {}, val_p = {}, ref_val = {}, ref_val_p = {}",
        //     &guan_pinned.as_ref().val,
        //     &guan_pinned.as_ref().val as *const String as usize,
        //     unsafe { &*(guan_pinned.as_ref().ref_val) },
        //     guan_pinned.as_ref().ref_val as usize,
        // );
    }

    #[test]
    fn test_move_action() {
        let mut s = String::from("hello world 1");
        let mut s1 = String::from("hello world 2");
        println!("{}", &s as *const String as usize);

        std::mem::swap(&mut s, &mut s1);
        println!("{}", &s as *const String as usize);
    }

}
