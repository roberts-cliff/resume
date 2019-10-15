# {{name}}
> {{contact}}

---
#### Relevant Skills
{{#each skills}}
- {{this}}{{/each}}

{{#each jobs}}
---
#### {{company}}
> {{tenure.start}} - {{tenure.end}}

{{#each roles}}
#### {{name}}
{{description}}
>{{period.start}} - {{period.end}}
>Tools and Technologies: {{#each bullets}}
>> - {{this}}{{/each}}
{{/each}}
{{/each}}
