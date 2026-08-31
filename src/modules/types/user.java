class User {
    @NonNull
    private String name;
    public User(@NonNull final String name) {
        this.name = name;
    }
    public void rename(@NonNull final String newName) {
        this.name = newName;
    }
    /**
     *  creates a new User instance from a UserProfile object. The name of the user is taken from the UserProfile's name.
     *  @param profile the UserProfile object from which to create the User instance
     *  @throws NullPointerException if the UserProfile's name is null
     */
    // these are so common typicall dont write as null pointers are so common in java
    public static User fromUserProfile(@NonNull final UserProfile profile) {
        String displayName = Objects.requireNonNull(profile.getName());
        return new User(profile.getName());
    }
}

// now imagine getiing every one to follow this rules, that not going to happen know lets see how its handled in rust
// we havent event mentioned weekly typed languages which are dumbster

// how we handle the same in rust?
