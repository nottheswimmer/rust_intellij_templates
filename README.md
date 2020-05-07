# rust_intellij_templates

A template to allow quick bootstrapping of Rust structs from IDEs
that support apache velocity templates (e.g. any JetBrains IDE that
supports Rust and maybe others).

## Installation
1. Go to File | Settings | Editor | File and Code Templates in JetBrains IDE (paste into browser, may require jetbrains toolbox)
   - CLion: jetbrains://CLion/settings?name=Editor--File+and+Code+Templates
   - IntelliJ IDEA: jetbrains://idea/settings?name=Editor--File+and+Code+Templates
   - PyCharm: jetbrains://Python/settings?name=Editor--File+and+Code+Templates
2. On the files tab, click +
3. Enter the following:
    - Name: My Rust Template (or whatever you want)
    - Extension: rs
    - Body: <paste `rust_struct_template.rs.vm` contents or .vm of your choice>
    - Reformat according to style: false
    - Enable Live Templates: true
4. Click OK 

## Usage
1. Right click -> New -> My Rust Template
2. Enter a filename
3. All other fields accept comma delimited parameters and may be empty.

### Example

The following input was used to generate [the person.rs file in the src directory](src/person.rs).

![Creating](screenshots/person.png?raw=true "Optional Title")