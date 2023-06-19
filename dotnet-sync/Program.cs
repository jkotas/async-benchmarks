using System.Diagnostics;
using System.Threading;

for (int i = 0; i < 10; i++)
{
    var sw = new Stopwatch();
    sw.Start();
    int result = A(100000);
    Console.WriteLine($"{sw.ElapsedMilliseconds} ms result={result}");
}

static int A(int n)
{
    int result = 1;
    for (int i = 1; i < n; i++)
        result = (result + i) * B(i);
    return result;
}

static int B(int n)
{
    int result = 0;
    for (int i = 0; i < n; i++)
    {
        if (result == 1_000_000)
            Thread.Yield();
        result += i * i;
    }
    return result;
}