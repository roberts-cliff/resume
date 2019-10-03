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

{{#each projects}}
#### {{name}}
{{description}}
>{{period.start}} - {{period.end}}
>Tools and Technologies: {{#each blurbs}}
>> - {{this}}{{/each}}
{{/each}}
{{/each}}
