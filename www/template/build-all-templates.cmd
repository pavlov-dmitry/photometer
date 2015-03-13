for /R %%F in (*.hbs) do (
    call build-template.cmd %%F
)