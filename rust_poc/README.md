# Possible challenges
- Database connection
- Template Compilation and Population
- getting config values
  - from a config file
    - in context with these individual features that can be enabled, we could still have a single config object, but with a lot of Option's
      - a alias to Option with a different name should be created, that indicates that this value is Required, but defined as "Option" for the fallback
  - from the database
- webserver that i like
- integration of threads from inputs, with threads from the webserver
- dealing with reduced modes of operation where some features do not exist (example, setup)
  - maybe a phase system, where progressively more and more systems and features are enabled and fallback back down
- returning to higher modes once something is restored
- twitch client

# Order of objectives to investigate in the prof of concept
1. Finding a Webserver that i like
   1. Implement Header based Authorization
2. Creating one easier input (NOT TWITCH)
3. Integration different controlflow directions (webserver + inputs)
4. Implementing Reduced modes
5. Implementing automatic upgrade of mode once available

# Solutions & References
https://www.arewewebyet.org/topics/templating
  - for out templating language
  - checked tera and handlebars
  - both support runtime created templates
  - didn't check any other once's