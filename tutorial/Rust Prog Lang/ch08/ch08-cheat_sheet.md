# Common Collections

Rust’s standard library includes a number of very useful data structures called collections that can contain multiple values. 

Unlike the built-in array and tuple types, the data that these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs. 

Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time. 

In this chapter, we’ll discuss three collections:
1. A vector allows you to store a variable number of values next to each other.
2. A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter, we’ll talk about it in depth.
3. A hash map allows you to associate a value with a specific key. It’s a particular implementation of the more general data structure called a map.



We’ll discuss how to create and update vectors, strings, and hash maps, as well as what makes each special.

This chapter contains: 
- [Storing Lists of Values with Vectors](./Vectors.md)
- [Storing UTF-8 Encoded Text with Strings](./Strings_UTF-8.md)
- [Storing Keys with Associated Values in Hash Maps](./Hash_Maps.md)

You can also check out [collections](https://doc.rust-lang.org/std/collections/index.html) provided by the standard library.

## Post Summary

Some exercises you should now be equipped to solve:

<table cellpadding="0" cellspacing="0"  bgcolor="#090909"   border="1"
  frame="box"   rules="none">
  <tr>
    <td width="66" valign="middle">
      <img
        src="../../../images/ed-images/ed_hack_1.png"
        width="40"
        height="40"
        alt="Author Git Logo"
        align="center"
      >
    </td>
    <td>
   <strong>My TODO IS better. Run test to see how vector of enums of different types allocates memory to ensure enough space at runtime - does it just allocate the largest type, or does compiler look at usage in the actual code?
</strong><br> println!("{}", std::mem::size_of::<SpreadsheetCell>());
    </td>
  </tr>
</table>

The standard library API documentation describes methods that vectors, strings, and hash maps have that will be helpful for these exercises!

1. Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
2. Convert strings to Pig Latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
3. Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then, let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

