import{_ as n,o as s,c as e,ag as t}from"./chunks/framework.BY_YGHoI.js";const h=JSON.parse('{"title":"Functions","description":"","frontmatter":{},"headers":[],"relativePath":"reference/functions.md","filePath":"reference/functions.md"}'),p={name:"reference/functions.md"};function l(i,a,r,c,o,d){return s(),e("div",null,[...a[0]||(a[0]=[t(`<h1 id="functions" tabindex="-1">Functions <a class="header-anchor" href="#functions" aria-label="Permalink to &quot;Functions&quot;">​</a></h1><h2 id="declaration" tabindex="-1">Declaration <a class="header-anchor" href="#declaration" aria-label="Permalink to &quot;Declaration&quot;">​</a></h2><div class="language-mlang vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang">mlang</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>&lt;return_type&gt; &lt;name&gt;(&lt;type&gt; &lt;param&gt;, ...) {</span></span>
<span class="line"><span>    # body</span></span>
<span class="line"><span>    return &lt;expr&gt;;</span></span>
<span class="line"><span>}</span></span></code></pre></div><p>Functions are declared at the top level, outside <code>main</code>. All parameters and the return type are explicitly typed.</p><div class="language-mlang vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang">mlang</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>int add(int a, int b) {</span></span>
<span class="line"><span>    return a + b;</span></span>
<span class="line"><span>}</span></span>
<span class="line"><span></span></span>
<span class="line"><span>txt greet(txt name) {</span></span>
<span class="line"><span>    return &quot;Hello, &quot; + name;</span></span>
<span class="line"><span>}</span></span>
<span class="line"><span></span></span>
<span class="line"><span>void log(txt msg) {</span></span>
<span class="line"><span>    print(msg);</span></span>
<span class="line"><span>}</span></span></code></pre></div><h2 id="calling-functions" tabindex="-1">Calling functions <a class="header-anchor" href="#calling-functions" aria-label="Permalink to &quot;Calling functions&quot;">​</a></h2><div class="language-mlang vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang">mlang</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>main() {</span></span>
<span class="line"><span>    print(add(3, 4));       # 7</span></span>
<span class="line"><span>    print(greet(&quot;MLang&quot;));  # Hello, MLang</span></span>
<span class="line"><span>    log(&quot;done&quot;);</span></span>
<span class="line"><span>}</span></span></code></pre></div><h2 id="return-types" tabindex="-1">Return types <a class="header-anchor" href="#return-types" aria-label="Permalink to &quot;Return types&quot;">​</a></h2><table tabindex="0"><thead><tr><th>Type</th><th>Meaning</th></tr></thead><tbody><tr><td><code>int</code></td><td>Returns a 64-bit integer</td></tr><tr><td><code>dec</code></td><td>Returns a decimal</td></tr><tr><td><code>txt</code></td><td>Returns a string</td></tr><tr><td><code>bool</code></td><td>Returns a boolean</td></tr><tr><td><code>array&lt;T&gt;</code></td><td>Returns an array</td></tr><tr><td><code>StructName</code></td><td>Returns an instance of a struct</td></tr><tr><td><code>void</code></td><td>No return value</td></tr></tbody></table><p>A <code>void</code> function may omit <code>return</code>, or use <code>return;</code> (not yet implemented — just omit it).</p><h2 id="recursion" tabindex="-1">Recursion <a class="header-anchor" href="#recursion" aria-label="Permalink to &quot;Recursion&quot;">​</a></h2><p>Functions can call themselves directly:</p><div class="language-mlang vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang">mlang</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>int factorial(int n) {</span></span>
<span class="line"><span>    if n &lt;= 1 {</span></span>
<span class="line"><span>        return 1;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>    return n * factorial(n - 1);</span></span>
<span class="line"><span>}</span></span>
<span class="line"><span></span></span>
<span class="line"><span>int fib(int n) {</span></span>
<span class="line"><span>    if n &lt;= 1 {</span></span>
<span class="line"><span>        return n;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>    return fib(n - 1) + fib(n - 2);</span></span>
<span class="line"><span>}</span></span></code></pre></div><h2 id="struct-typed-parameters-and-return-values" tabindex="-1">Struct-typed parameters and return values <a class="header-anchor" href="#struct-typed-parameters-and-return-values" aria-label="Permalink to &quot;Struct-typed parameters and return values&quot;">​</a></h2><div class="language-mlang vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang">mlang</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>struct Vec2 {</span></span>
<span class="line"><span>    dec x</span></span>
<span class="line"><span>    dec y</span></span>
<span class="line"><span>}</span></span>
<span class="line"><span></span></span>
<span class="line"><span>Vec2 scale(Vec2 v, dec factor) {</span></span>
<span class="line"><span>    return Vec2 { x = v.x * factor, y = v.y * factor };</span></span>
<span class="line"><span>}</span></span>
<span class="line"><span></span></span>
<span class="line"><span>main() {</span></span>
<span class="line"><span>    let v = Vec2 { x = 1.0, y = 2.0 };</span></span>
<span class="line"><span>    let doubled = scale(v, 2.0);</span></span>
<span class="line"><span>    print(doubled.x);   # 2.0</span></span>
<span class="line"><span>}</span></span></code></pre></div><h2 id="scope" tabindex="-1">Scope <a class="header-anchor" href="#scope" aria-label="Permalink to &quot;Scope&quot;">​</a></h2><ul><li>Functions have their own isolated scope.</li><li>Parameters are passed by value (structs and arrays are copied).</li><li>Functions cannot access variables from the caller&#39;s scope.</li></ul><h2 id="forward-references" tabindex="-1">Forward references <a class="header-anchor" href="#forward-references" aria-label="Permalink to &quot;Forward references&quot;">​</a></h2><p>All non-<code>main</code> functions and struct declarations are registered before execution begins, so declaration order does not matter.</p>`,19)])])}const g=n(p,[["render",l]]);export{h as __pageData,g as default};
