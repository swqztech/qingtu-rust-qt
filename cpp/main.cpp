#include "main_window.h"

#include <QApplication>

int main(int argc, char* argv[]) {
  QApplication app(argc, argv);
  QApplication::setApplicationName(QStringLiteral("qingtu-rust-qt"));
  MainWindow w;
  w.show();
  return app.exec();
}
