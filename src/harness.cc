#include <xlnt/xlnt.hpp>
#include <libstudxml/parser.hxx>
#include <unistd.h>
__AFL_FUZZ_INIT();
int main(int argc, char **argv) {
    int len = __AFL_FUZZ_TESTCASE_LEN;
    unsigned char *buf = __AFL_FUZZ_TESTCASE_BUF;
    std::vector<uint8_t> v_data(buf, buf + len);
    xlnt::workbook excelWorkbook;
    try
    {
        excelWorkbook.load(v_data);
    }
    catch (const xlnt::exception& e)
    {
        return 0;
    }
    catch (const xml::parsing& e)
    {
        return 0;
    }
    return 0;
}
