pub static ERRORS: &[(u32, &str)] = &[
    (1001, "Invalid input"),
    (1002, "User not found"),
    (1003, "Permission denied"),
    (1004, "Something went wrong"),

    (400000, "Bad request"),
    (400001, "Username already in use"),
    (400002, "Email already in use"),
    (400004, "Domain already in use"),
    (400005, "Data not found"),


    (401000, "Unauthorized request"),
    (401001, "Username not found"),
    (401002, "Username or password are incorrect"),
    (401003, "Username already in use"),
    (401004, "Token Error"),

    (403000, "Forbidden access"),
    (403001, "Role not found"),


    (500002, "Cannot email"),
];