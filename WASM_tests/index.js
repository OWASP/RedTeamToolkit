const rust = import('./pkg');

rust
  .then(m => m.say_hi("sam"))
  .catch(console.error);