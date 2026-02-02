```html
<script src="index.js" type="text/javascript"></script>

```
```javascript
cons person = {
    isHuman: false,
    printIntroduction: function () {
        console.log(`My name is ${this.name}. Am I human ${this.isHuman}`);
    },
};

cons me = Object.create(person);

me.isHuman = true;
me.name = 'moe';
me.printIntroduction();

```
