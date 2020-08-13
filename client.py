import not_alone
from PyQt5 import QtWidgets, QtCore, QtGui
import time

class MessageHandler(QtCore.QThread):
    new_message = QtCore.pyqtSignal(str)

    def __init__(self, client):
        QtCore.QThread.__init__(self, None)
        self.client = client
    
    def run(self):
        while True:
            text = self.client.read_message()
            if text != "":
                self.new_message.emit(text)
            else:
                time.sleep(0.5)



class ClientWind(QtWidgets.QWidget):
    
    def __init__(self, name, parent=None):
        QtWidgets.QWidget.__init__(self, parent=parent)
        self.name = name
        self.messages_viev = QtWidgets.QLabel()
        self.text_inp = QtWidgets.QLineEdit()
        self.send_btn = QtWidgets.QPushButton("Отправить сообщение")
        self.send_btn.clicked.connect(self.send_message)

        layaot = QtWidgets.QVBoxLayout()
        layaot.addWidget(self.messages_viev)
        layaot.addWidget(self.text_inp)
        layaot.addWidget(self.send_btn)
        self.setLayout(layaot)
        self.show()

    def connect_server(self, addr):
        self.client = not_alone.client.Client(addr)
        self.handler = MessageHandler(self.client)
        self.handler.new_message.connect(lambda msg: self.add_messages(msg))
        self.handler.start()

    def add_messages(self, message):
        self.messages_viev.setText(self.messages_viev.text() + message + "\n") 

    def send_message(self):
        self.client.send_message(f"[{self.name}]:" + self.text_inp.text())
        self.add_messages("[Me]:" + self.text_inp.text())
        self.text_inp.setText("")


if __name__ == "__main__":
    app = QtWidgets.QApplication([])
    a = ClientWind("Vadim")
    a.connect_server("localhost:9090")
    app.exec()
