<style 
  type="text/css" rel="stylesheet">
#limheight {
    height: 300px; /*your fixed height*/
    -webkit-column-count: 3;
       -moz-column-count: 3;
            column-count: 3; /*3 in those rules is just placeholder -- can be anything*/
}

#limheight li {
    display: inline-block; /*necessary*/
}
</style>
# {{name}}
> {{contact}}

---
#### Relevant Skills
<ul id="limheight">{{#each skills}}
<li>{{this}}</li>{{/each}}
</ul>

{{#each jobs}}
---
#### {{company}}
> {{tenure.start}} - {{tenure.end}}

{{#each projects}}
#### {{name}}
>{{period.start}} - {{period.end}}

{{description}}
>Tools and Technologies: {{#each stack}}
>> - {{tech}}{{/each}}
{{/each}}
{{/each}}