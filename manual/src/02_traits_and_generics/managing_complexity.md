# Managing Complexity

Traits and generics are really powerful, so there's a temptation to keep stacking them. If you look at the source for Axum or the Bevy Engine, you'll find some masterpieces of programming---that aren't as readable as you might like. Bevy, for example, allows you to define functions as "systems" that accept up to 32 parameters, each of which can be a different type (and are automatically wired up to dependency injection).

Just in case 32 wasn't enough, it can also take 32 TUPLES of up to 32 parameters each.

> If you need that many parameters, refactor!