It's a simple schedule bot, hooray!

You need usual discord stuff, get token, save to .env, run and add bot to your server. Right now it expectes schedule like this (need fixes cause it's very specific):

```
you can write anything you want here
but you need to start schedule entries with **schedule** like below V

**schedule**
TUESDAY 07/05 9PM - The label whatever you want
... more schedule days entries

^ whitespace indicates end of schedule
you can add anything else you want after that too
```

That's it! After it's saved people can use `/show_schedule` and `/next_stream` to get info.

BTW, there are no limits to `/save_schedule`, have fun with that!
