#[macro_export]
macro_rules! example {
    ($title:literal | $($t:tt)*) => {
        html! {
            <>
                <h2>{$title}</h2>

                <h3>{"Example"}</h3>

                $($t)*

                <h3>{"Code"}</h3>

                <div class="pf-c-code-editor">
                    <div class="pf-c-code-editor__main">
                        <div class="pf-c-code-editor__code">
                            <pre class="pf-c-code-editor__code-pre">
                                {"html! {\n"}
                                    {stringify!($($t)*)}
                                {"\n}"}
                            </pre>
                        </div>
                    </div>
                </div>
            </>
        }
    };
}
