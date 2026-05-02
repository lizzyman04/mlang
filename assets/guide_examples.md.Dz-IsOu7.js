import{_ as n,o as s,c as p,ag as e}from"./chunks/framework.BY_YGHoI.js";const h=JSON.parse('{"title":"Examples","description":"","frontmatter":{},"headers":[],"relativePath":"guide/examples.md","filePath":"guide/examples.md"}'),l={name:"guide/examples.md"};function i(t,a,c,o,r,d){return s(),p("div",null,[...a[0]||(a[0]=[e(`<h1 id="examples" tabindex="-1">Examples <a class="header-anchor" href="#examples" aria-label="Permalink to &quot;Examples&quot;">​</a></h1><h2 id="fibonacci" tabindex="-1">Fibonacci <a class="header-anchor" href="#fibonacci" aria-label="Permalink to &quot;Fibonacci&quot;">​</a></h2><div class="language-mlang vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang">mlang</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>int fib(int n) {</span></span>
<span class="line"><span>    if n &lt;= 1 {</span></span>
<span class="line"><span>        return n;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>    return fib(n - 1) + fib(n - 2);</span></span>
<span class="line"><span>}</span></span>
<span class="line"><span></span></span>
<span class="line"><span>main() {</span></span>
<span class="line"><span>    for i in 0..10 {</span></span>
<span class="line"><span>        print(fib(i));</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>}</span></span></code></pre></div><h2 id="factorial" tabindex="-1">Factorial <a class="header-anchor" href="#factorial" aria-label="Permalink to &quot;Factorial&quot;">​</a></h2><div class="language-mlang vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang">mlang</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>int factorial(int n) {</span></span>
<span class="line"><span>    if n &lt;= 1 {</span></span>
<span class="line"><span>        return 1;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>    return n * factorial(n - 1);</span></span>
<span class="line"><span>}</span></span>
<span class="line"><span></span></span>
<span class="line"><span>main() {</span></span>
<span class="line"><span>    print(factorial(10));   # 3628800</span></span>
<span class="line"><span>}</span></span></code></pre></div><h2 id="arrays" tabindex="-1">Arrays <a class="header-anchor" href="#arrays" aria-label="Permalink to &quot;Arrays&quot;">​</a></h2><div class="language-mlang vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang">mlang</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>main() {</span></span>
<span class="line"><span>    array&lt;int&gt; nums = [10, 20, 30, 40, 50];</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    print(nums.len());          # 5</span></span>
<span class="line"><span>    nums.push(60);</span></span>
<span class="line"><span>    print(nums.len());          # 6</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    for n in nums {</span></span>
<span class="line"><span>        print(n);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    let first_three = nums.slice(0, 3);</span></span>
<span class="line"><span>    print(first_three);         # [10, 20, 30]</span></span>
<span class="line"><span>}</span></span></code></pre></div><h2 id="structs" tabindex="-1">Structs <a class="header-anchor" href="#structs" aria-label="Permalink to &quot;Structs&quot;">​</a></h2><div class="language-mlang vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang">mlang</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>struct Point {</span></span>
<span class="line"><span>    int x</span></span>
<span class="line"><span>    int y</span></span>
<span class="line"><span>}</span></span>
<span class="line"><span></span></span>
<span class="line"><span>Point midpoint(Point a, Point b) {</span></span>
<span class="line"><span>    return Point { x = (a.x + b.x) / 2, y = (a.y + b.y) / 2 };</span></span>
<span class="line"><span>}</span></span>
<span class="line"><span></span></span>
<span class="line"><span>main() {</span></span>
<span class="line"><span>    let p1 = Point { x = 0, y = 0 };</span></span>
<span class="line"><span>    let p2 = Point { x = 10, y = 6 };</span></span>
<span class="line"><span>    let mid = midpoint(p1, p2);</span></span>
<span class="line"><span>    print(mid.x);   # 5</span></span>
<span class="line"><span>    print(mid.y);   # 3</span></span>
<span class="line"><span>}</span></span></code></pre></div><h2 id="interactive-i-o" tabindex="-1">Interactive I/O <a class="header-anchor" href="#interactive-i-o" aria-label="Permalink to &quot;Interactive I/O&quot;">​</a></h2><div class="language-mlang vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang">mlang</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>main() {</span></span>
<span class="line"><span>    txt name = read(&quot;Enter your name: &quot;);</span></span>
<span class="line"><span>    int age  = int(read(&quot;Enter your age: &quot;));</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    print(&quot;Hello, &quot; + name + &quot;!&quot;);</span></span>
<span class="line"><span>    print(&quot;Next year you will be &quot; + (age + 1));</span></span>
<span class="line"><span>}</span></span></code></pre></div><h2 id="fizzbuzz" tabindex="-1">FizzBuzz <a class="header-anchor" href="#fizzbuzz" aria-label="Permalink to &quot;FizzBuzz&quot;">​</a></h2><div class="language-mlang vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang">mlang</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>main() {</span></span>
<span class="line"><span>    for i in 1..101 {</span></span>
<span class="line"><span>        bool fizz = i % 3 == 0;</span></span>
<span class="line"><span>        bool buzz = i % 5 == 0;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        if fizz &amp;&amp; buzz {</span></span>
<span class="line"><span>            print(&quot;FizzBuzz&quot;);</span></span>
<span class="line"><span>        } else {</span></span>
<span class="line"><span>            if fizz {</span></span>
<span class="line"><span>                print(&quot;Fizz&quot;);</span></span>
<span class="line"><span>            } else {</span></span>
<span class="line"><span>                if buzz {</span></span>
<span class="line"><span>                    print(&quot;Buzz&quot;);</span></span>
<span class="line"><span>                } else {</span></span>
<span class="line"><span>                    print(i);</span></span>
<span class="line"><span>                }</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>}</span></span></code></pre></div><blockquote><p>All example files live in the <code>examples/</code> directory of the repository.</p></blockquote>`,14)])])}const m=n(l,[["render",i]]);export{h as __pageData,m as default};
