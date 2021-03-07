using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Collections.Generic;
using Day1;

namespace Day1T
{
    [TestClass]
    public class UnitTest1
    {
        [TestMethod]
        public void TestMethod1()
        {
            var tests = new List<(string, int)> { ("1122", 3), ("1111", 4), ("1234", 0), ("91212129", 9) };

            foreach (var (input, output) in tests)
            {
                Assert.AreEqual(Day1.Day1.SeqSum(input, false), output);
            }
        }

        [TestMethod]
        public void TestMethod2()
        {
            var tests = new List<(string, int)> { ("1212", 6), ("1221", 0), ("123425", 4), ("123123", 12), ("12131415", 4) };

            foreach (var (input, output) in tests)
            {
                Assert.AreEqual(Day1.Day1.SeqSum(input, true), output);
            }
        }
    }
}
