﻿using System.Diagnostics;
using System.Runtime.CompilerServices;

const uint Threshold = 20_000_000;

for (int i = 0; i < 10; i++)
{
    var sw = new Stopwatch();
    sw.Start();
    uint result = await A(100_000_000);
    Console.WriteLine($"{sw.ElapsedMilliseconds} ms result={result}");
}

static async ValueTask<uint> A(uint n)
{
    uint result = n;
    for (uint i = 0; i < n; i++)
        result = await B(result);
    return result;
}

static async ValueTask<uint> B(uint n)
{
    uint result = n;

    result = result * 1_999_999_981;
    if (result < Threshold)
        await Task.Yield();

    result = result * 1_999_999_981;
    if (result < Threshold)
        await Task.Yield();

    result = result * 1_999_999_981;
    if (result < Threshold)
        await Task.Yield();

    return result;
}