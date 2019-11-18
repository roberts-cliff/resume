# {{name}}
> {{contact}}

{{purpose}}
---
#### Relevant Skills
{{#each skills}}
- {{this}}{{/each}}
{{#each jobs}}
---
#### {{company}}
> {{tenure.start}} - {{tenure.end}}
>{{#each roles}}
>{{description}} {{period.start}} - {{period.end}}
{{#each bullets}}>>> - {{this}}
{{/each}}
{{/each}}
{{/each}}
#### Education
{{#each education}}
 - {{this}}{{/each}}
---
References available on request.
