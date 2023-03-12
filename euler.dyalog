seive ← { ⍝ primes up to n
    n ← ⍵
    ns ← 2,1+2×⍳n ⍝ odd numbers up to n, including 2
    pr ← {⍵, ⊃((∧⌿0≠⍵ ∘.|ns) / ns)}⍣{¯1↑⍺ > n} 2 3 ⍝ prime_sive up to n
    pr
}
larget_prime_factor_2 ← { ⍝ Does not work for large numbers and is super slow due to prime seive!
    pr ← prime_sive ⍵÷2
    ¯1↑(0=pr|⍵) / pr
}

q1 ← ⊢ +/⍤/ (∨ ⌿ 0=3 5 ∘.| ⊢)
q2 ← {
    fib ← 1↓{⍵, ⍨+/ 2↑⍵}⍣{⊃⍺>4000000} 2 1
    f ← ⊢ +/⍤/ 0=2|⊢
    f fib
}
 p3←{
    ⍝ ⍵ ← 600851475143
    ns←1↓⍳⌊1+(⍵*÷2) ⍝ array of numbers up to n/2
    fs←(0=ns|⍵)/ns ⍝ all odd factors (with 2) of n up to sart(n), without 1
    fs←fs,⍵÷(0=fs|⍵)/fs ⍝ all factors of n 
    ¯1↑fs/⍨(1=+/[1](0=fs∘.|fs))  ⍝ largest prime factor of n
 }
p4 ← {
    ⍝ ⍵ ← 99↓⍳999
    f ← (6⍴10)⊤⊢ ⍝ convert 6 digit number into a 1x6 vector
    m ← f∘.×⍨⍵ ⍝ apply conversion to each product of two 3-digit numbers
    r ← ⊖m ⍝ rotate the 900x900x6 matrix that encodes each product
    ⌈/(,∧⌿m=r)/,10⊥m ⍝ find the largest product whose vector encoding is the same normally as when rotated
}
p5 ← {
    ∧⌿⍳20
}
p6 ← {
    (2*⍨+/ns) - (+/ns*2)
}
p7 ← { ⍝ primes up to n
     ns←1↓⍳⍵
     s ← ⌊2÷⍨⍵*÷2
     os←2,1+2×⍳s ⍝ odd numbers up to n/2, including 2
     pr←ns/⍨(∧⌿0≠os∘.|ns)
     pr2 ← seive s×2
     (pr2/⍨(pr2 ∊ os)), pr
 }