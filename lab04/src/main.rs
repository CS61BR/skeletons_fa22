use flik::is_same_number;

mod flik;

fn main() {
    let mut user_entered = 9001;
    let mut super_secret_password = 42;

    println!(
        "Checking if user-entered value {user_entered} is the same as \
              the super-secret-password {super_secret_password}"
    );

    // to protect the security of the super-secret-password, put both values
    // through some transformations before letting Flik Enterprises code see them
    user_entered *= 1234567;
    super_secret_password *= 1234567;
    user_entered += 97;
    super_secret_password += 97;

    if is_same_number(user_entered, super_secret_password) {
        println!("Correct. Access Granted.");
    } else {
        println!("Incorrect. Access Denied.");
    }
}
