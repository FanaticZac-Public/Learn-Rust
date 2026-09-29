

## Extending Cargo with Custom Commands

Cargo is designed so that you can extend it with new subcommands without having to modify it. If a binary in your $PATH is named cargo-something, you can run it as if it were a Cargo subcommand by running cargo something

Custom commands like this are also listed when you run cargo --list. Being able to use cargo install to install extensions and then run them just like the built-in Cargo tools is a super-convenient benefit of Cargo’s design!

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
 <strong>
Prove it!

Just kidding - I showed this with ripgrep in last section.
</strong>
    </td>
  </tr>
</table>


