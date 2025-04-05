
# 🌟 Xinux Shell

Welcome to **Xinux Shell**, the command-line interface (CLI) that’s so good, it might just make you forget about GUIs altogether. Whether you're a developer, a power user, or someone who just likes to type random commands to feel productive, Xinux Shell is here to revolutionize your terminal experience. Seriously, it’s like upgrading from a tricycle to a rocket ship. 🚀

---

## 🚀 Key Features

### 1️⃣ **Customizable Prompt Styles**
- Why settle for boring prompts when you can have **15+ stunning styles**? Pick one that screams *you*—or at least whispers it politely.
    - **Examples:** 
        - Single-line (for minimalists)
        - Two-line (for overachievers)
        - Boxed (because why not?)
        - Minimal (less is more)
        - Classic (nostalgia, anyone?)
        - Bold Frame (for the dramatic)
        - Cyberpunk (neon dreams)
        - Retro (hello, 80s!)
- Oh, and did we mention they’re **colorized** and adapt to your current directory? Your terminal will look so good, you might start showing it off at parties.

---

### 2️⃣ **Intelligent Auto-Completion**
- Typing is hard. Let Xinux Shell do the heavy lifting with **context-aware auto-completion** for:
    - Shell history (because who remembers what they typed 5 minutes ago?)
    - Built-in commands (like `cd`, `ls`, and other classics)
    - Executables in your `$PATH` (no more guessing games)
    - User-defined aliases (your shortcuts, your rules)
- And if you mess up? Don’t worry—**hints for incomplete commands** will gently nudge you in the right direction. It’s like having a helpful friend who doesn’t judge.

---

### 3️⃣ **Advanced History Management**
- Forget the days of scrolling through endless, messy command history. Xinux Shell keeps things tidy:
    - **Duplicate-free history tracking** ensures no command gets repeated unnecessarily. (We’re looking at you, `ls`.)
    - Want a clean slate? Just type `clear history` and poof—gone.
- Your history is safely stored in:
  ```plaintext
  ~/.config/.Xinux/history.txt
  ```
- Never lose track of your most-used commands again. Unless you want to, of course.

---

### 4️⃣ **Powerful Aliases**
- Tired of typing the same long commands over and over? Create **custom shortcuts** with the `alias` command.
    - **Usage:**
      ```bash
      alias name=command
      ```
    - **Example:**
      ```bash
      alias ll="ls -la"
      ```
- Aliases are saved for eternity (or until you delete them) in the configuration file. Your fingers will thank you.

---

### 5️⃣ **Built-In Commands**
Xinux Shell comes preloaded with **essential built-in commands** that make you wonder how you ever lived without them:
| Command               | Description                                                                 |
|-----------------------|-----------------------------------------------------------------------------|
| `cd`                 | Change the current directory.                                               |
| `ls`                 | List files and directories.                                                 |
| `clear`              | Clear the terminal screen (because chaos isn’t always fun).                 |
| `exit` / `quit`      | Leave the shell (but we’ll miss you).                                       |
| `help`               | Get a list of commands and feel like a genius.                              |
| `echo`               | Print text to the terminal (because why not?).                              |
| `xinsay`             | Like `cowsay`, but cooler.                                                  |
| `alias`              | Create shortcuts for commands.                                              |
| `clear history`      | Wipe your command history clean.                                            |
| `xinux config prompt`| Change your prompt style on the fly.                                        |

---

### 6️⃣ **First-Time Setup**
- First impressions matter, and Xinux Shell knows it. On your **first launch**, you’ll get a guided setup to:
    - Choose your favorite **prompt style** (good luck picking just one).
    - Configure initial settings like a pro.
- All your preferences are saved to:
  ```plaintext
  ~/.config/.Xinux/config.toml
  ```
- It’s like a warm welcome, but for your terminal.

---

### 7️⃣ **Dynamic Configuration**
- Want to tweak your setup? No problem. Just edit the configuration file:
  ```plaintext
  ~/.config/.Xinux/config.toml
  ```
- Customize everything from **prompt styles** to **aliases**. And the best part? Changes are applied instantly—no need to restart. (Take that, other shells!)

---

---

So, what are you waiting for? Start your journey with **Xinux Shell** today and turn your terminal into a productivity powerhouse. Or at least make it look really cool. 🚀

> Pro tip: When in doubt, type `help`. It’s like having a cheat sheet, but without the guilt.

## Shhhh... Xinsay has something to tell you. Type `xinsay` to find out.
### Also, don’t forget to check out [CONFIG](CONFIG.md) for all the juicy customization options.

