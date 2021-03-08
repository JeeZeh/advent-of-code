using Day2;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace Day2T
{
    [TestClass]
    public class UnitTest1
    {
        [TestMethod]
        public void TestMethod1()
        {
            var spreadsheet = new Spreadsheet(Resources.sample);
            
            Assert.AreEqual(18, spreadsheet.GetChecksum());
        }
        
        [TestMethod]
        public void TestMethod2()
        {
            var spreadsheet = new Spreadsheet(Resources.sample2);
            
            Assert.AreEqual(9, spreadsheet.GetChecksumV2());
        }
    }
}
