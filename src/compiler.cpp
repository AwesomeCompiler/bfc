#include "llvm/IR/Verifier.h"
#include "llvm/IR/DerivedTypes.h"
#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/LLVMContext.h"
#include "llvm/IR/Module.h"
#include "llvm/Support/raw_ostream.h"
#include "llvm/Support/raw_os_ostream.h"

#include <boost/filesystem.hpp>

#include <iostream>
#include <fstream>
#include <regex>

#include "parser.hpp"
#include "bfir.hpp"
#include "optimisations.hpp"

using namespace llvm;

void printUsage(std::string ProgramName) {
    errs() << "Usage: " << ProgramName << " <my-program.bf> \n";
}

std::string getOutputName(std::string ProgramName) {
    // Strip the path, so "../foo/bar/baz.bf" -> "baz.bf".
    std::regex FilenamePattern("[^/]+$");
    std::smatch RegexMatch;
    std::regex_search(ProgramName, RegexMatch, FilenamePattern);
    std::string FileName = RegexMatch[0].str();

    // Strip the extension "baz.bf" -> "baz"
    std::regex ExtensionPattern("\\.bf?$");
    std::string Replacement("");
    std::string Name =
        std::regex_replace(FileName, ExtensionPattern, Replacement);

    return Name + ".ll";
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        printUsage(argv[0]);
        return EXIT_FAILURE;
    }

    auto ProgramPath = std::string(argv[1]);

    if (!boost::filesystem::exists(ProgramPath)) {
        errs() << "No such file: " << ProgramPath << "\n";
        return EXIT_FAILURE;
    }

    auto Source = readSource(ProgramPath);
    auto Program = parseSource(Source);

    Program = combineIncrements(Program);
    Program = combineDataIncrements(Program);
    Program = markKnownZero(Program);
    Program = simplifyZeroingLoop(Program);
    Program = combineSetAndIncrements(Program);

    std::cout << Program << std::endl;

    Module *Mod = compileProgram(Program);

    // Write the LLVM IR to a file.
    std::ofstream StdOutputFile(getOutputName(ProgramPath));
    raw_os_ostream OutputFile(StdOutputFile);
    Mod->print(OutputFile, nullptr);

    delete Mod;

    return 0;
}
